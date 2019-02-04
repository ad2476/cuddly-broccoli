//! Safe wrappers for compiling and linking GLSL shader programs.
//!
//! Uses the `ResourceLoader` system to load shader sources from file.
//!
//! # Examples
//!
//! Load vertex and fragment shaders `assets/shader.vert`, `assets/shader.frag`.
//! ```
//! # use std::path::Path;
//! # use resources::ResourceLoader;
//! let loader: ResourceLoader = ResourceLoader::new(Path::new("assets/")).unwrap();
//!
//! let program = Program::from_res(&loader, "shader").unwrap();
//! ```

use gl;
use std::collections::HashMap;
use std::ffi::{CStr, CString, OsStr};
use std::path::{Path, PathBuf};

use rendergl::{uniform, UniformSet};
use resources::{self, ResourceLoader};

/// Error enum for shaders
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load {}", name)]
    ResourceLoadError {
        name: String,
        #[cause]
        inner: resources::Error,
    },
    #[fail(display = "Can not determine shader type for {}", name)]
    UnknownShaderType { name: String },
    #[fail(display = "Failed to compile shader {}:\n\t{}", name, message)]
    CompileError { name: String, message: String },
    #[fail(display = "Failed to link program {}: {}", name, message)]
    LinkError { name: String, message: String },
    #[fail(display = "Encoding invalid")]
    EncodingError,
}

/// Type alias for mapping uniform names to `GLint` identifiers.
type UniformMap = HashMap<String, Vec<gl::types::GLint>>;

/// Wraps OpenGL shader program object.
///
/// Stores a handle to the openGL object for a shader program,
/// and exposes safe methods on that object.
pub struct Program {
    id: gl::types::GLuint,
    uniforms: UniformMap,
}

impl Program {
    /// Construct a shader program from resource.
    ///
    /// Here, `name` assumes there exist vertex and fragment shaders within the resource system
    /// called `name.vert` and `name.frag`.
    ///
    /// `name` should be a relative path from the resource root.
    pub fn from_res(res: &ResourceLoader, name: &str) -> Result<Program, Error> {
        const EXTENSIONS: [&str; 2] = ["vert", "frag"];

        let mut resource_path = PathBuf::from(name);

        let shaders = EXTENSIONS
            .iter()
            .map(|ext| {
                resource_path.set_extension(ext);
                Shader::from_res(&res, &resource_path.as_path())
            })
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(&shaders[..]).map_err(|m| Error::LinkError {
            name: name.into(),
            message: m,
        })
    }

    /// Construct a shader program from a list of `Shader` structs.
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };
        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id);
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
        }

        if success == 0 {
            // error compiling shader
            let error = alloc_whitespace_cstring(len as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    ::std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }
            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id);
            }
        }

        let mut program = Program {
            id: program_id,
            uniforms: HashMap::new(),
        };
        program.discover_uniforms()?;
        Ok(program)
    }

    /// Use this program (safely calls `glUseProgram`).
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /// Stop using this program (safely calls `glUseProgram(0)`).
    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    /// Static way to call `unbind()`.
    pub fn bind_default() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    /// Set an element of a uniform array.
    ///
    /// For example, consider your shader contains a `uniform mat3 lights[10]`.
    /// ```
    /// # pass `light_data` to the third light
    /// program.set_uniform_by_index("lights", light_data, 2);
    /// ```
    pub fn set_uniform_by_index<T: UniformSet>(
        &self,
        name: &str,
        data: &T,
        index: usize,
    ) -> Result<(), uniform::Error> {
        let slots = self
            .uniforms
            .get(name)
            .ok_or(uniform::Error::NotFoundError {
                name: name.to_string(),
            })?;
        let loc = slots
            .get(index)
            .ok_or(uniform::Error::IndexError { index })?;
        Ok(data.set_uniform_gl(*loc))
    }

    /// Convenience wrapper for `set_uniform_by_index` for non-array uniforms.
    /// Always sets index `0`.
    pub fn set_uniform<T: UniformSet>(&self, name: &str, data: &T) -> Result<(), uniform::Error> {
        self.set_uniform_by_index(name, data, 0)
    }

    fn discover_uniforms(&mut self) -> Result<(), String> {
        let mut uniform_count: gl::types::GLint = 0;
        self.bind();
        unsafe {
            gl::GetProgramiv(self.id, gl::ACTIVE_UNIFORMS, &mut uniform_count);
        }
        for i in 0..uniform_count {
            let buffer_size: gl::types::GLsizei = 256;
            let mut name_length: gl::types::GLsizei = 0;
            let mut array_size: gl::types::GLsizei = 0;
            let mut dtype: gl::types::GLenum = 0;
            let name = alloc_nul_cstring(buffer_size as usize);
            unsafe {
                gl::GetActiveUniform(
                    self.id,
                    i as u32,
                    buffer_size,
                    &mut name_length,
                    &mut array_size,
                    &mut dtype,
                    name.as_ptr() as *mut gl::types::GLchar,
                );
            }
            self.add_uniform(&name, array_size as usize)?;
        }
        Program::bind_default();
        Ok(())
    }

    // Collect all elements of the uniform. Provides support for uniform arrays.
    fn add_uniform(&mut self, name: &CString, size: usize) -> Result<(), String> {
        // Create a new String,
        // remove any array symbols, trailing zeros from name:
        let clean_name = name
            .clone()
            .into_string()
            .map_err(|e| format!("{}", e))?
            .trim_right_matches(char::from(0))
            .replace("[0]", "");

        let mut array: Vec<gl::types::GLint> = Vec::with_capacity(size);
        // insert the first array slot - or scalar element
        array.push(unsafe { gl::GetUniformLocation(self.id, name.as_ptr()) });

        // if this is a uniform array, insert the next slots
        let array_size: u8 = ::std::cmp::min(size as u8, 9);
        let name_size = name.as_bytes().len();
        for i in 1..array_size {
            let mut enumerated_name = name.clone().into_bytes();
            enumerated_name[name_size - 2] = b'0' + i;
            array.push(unsafe {
                gl::GetUniformLocation(
                    self.id,
                    enumerated_name.as_ptr() as *const gl::types::GLchar,
                )
            });
        }

        println!("discovered uniform: {}", clean_name);
        self.uniforms.insert(clean_name, array);
        Ok(())
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

/// Wraps a shader source object loaded into OpenGL.
pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    /// Load shader source from resource.
    pub fn from_res(res: &ResourceLoader, path: &Path) -> Result<Shader, Error> {
        const EXT_TYPES: [(&str, gl::types::GLenum); 2] =
            [("vert", gl::VERTEX_SHADER), ("frag", gl::FRAGMENT_SHADER)];

        let name = path.to_str().ok_or(Error::EncodingError)?;
        let path_ext = path.extension().unwrap_or(OsStr::new(""));
        let shader_type = EXT_TYPES
            .iter()
            .find(|&&(ext, _)| ext == path_ext)
            .map(|&(_, s_type)| s_type)
            .ok_or(Error::UnknownShaderType { name: name.into() })?;

        let source = res
            .load_cstring(path)
            .map_err(|e| Error::ResourceLoadError {
                name: name.into(),
                inner: e,
            })?;

        Shader::from_source(&source, shader_type).map_err(|m| Error::CompileError {
            name: name.into(),
            message: m,
        })
    }

    /// Load shader source from null-terminated buffer.
    pub fn from_source(
        source: &CStr, // need null-terminated buffer
        shader_type: gl::types::GLuint,
    ) -> Result<Shader, String> {
        let id = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), ::std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        if success == 0 {
            // error compiling shader
            let error = alloc_whitespace_cstring(len as usize);
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    ::std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }
            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Shader { id })
    }

    /// Create vertex source from null-terminated buffer.
    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    /// Create fragment source  from null-terminated buffer.
    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn alloc_whitespace_cstring(len: usize) -> CString {
    let buf: Vec<u8> = vec![b' '; len];
    CString::new(buf).unwrap()
}

fn alloc_nul_cstring(len: usize) -> CString {
    let buf: Vec<u8> = vec![0; len];
    unsafe { CString::from_vec_unchecked(buf) }
}
