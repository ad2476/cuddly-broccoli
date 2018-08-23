use gl;
use std;
use std::ffi::{CString, CStr, OsStr};
use std::path::{Path, PathBuf};

use resources::{self, ResourceLoader};

/// Error enum for shaders
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load {}", name)]
    ResourceLoadError { name: String, #[cause] inner: resources::Error },
    #[fail(display = "Can not determine shader type for {}", name)]
    UnknownShaderType { name: String },
    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { name: String, message: String},
    #[fail(display = "Failed to link program {}: {}", name, message)]
    LinkError { name: String, message: String},
    #[fail(display = "Path encoding invalid")]
    EncodingError,
} 

/// Wraps OpenGL shader program object.
///
/// Stores a handle to the openGL object for a shader program,
/// and exposes safe methods on that object.
pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    /// Load a shader program from resource
    pub fn from_res(
        res: &ResourceLoader,
        name: &str
    ) -> Result<Program, Error> {
        const EXTENSIONS: [&str; 2] = ["vert", "frag"];

        let mut resource_path = PathBuf::from(name);

        let shaders = EXTENSIONS.iter()
            .map(|ext| {
                resource_path.set_extension(ext);
                Shader::from_res(&res, &resource_path.as_path())
            })
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(&shaders[..])
            .map_err(|m| Error::LinkError { name: name.into(), message: m })
    }

    /// Create a shader program from a list of `Shader` structs
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };
        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id); }
        }

        unsafe { gl::LinkProgram(program_id); }

        let mut success: gl::types::GLint = 1;
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
        }

        if success == 0 { // error compiling shader
            let error = alloc_whitespace_cstring(len as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id); }
        }

        Ok(Program { id: program_id })
    }

    /// Use this program (safely calls `glUseProgram`).
    pub fn set_used(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    /// Stop using this program (safely calls `glUseProgram(0)`).
    pub fn unset_used(&self) {
        unsafe { gl::UseProgram(0); }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}

/// Wraps a shader source object loaded into OpenGL.
pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    /// Load shader source from resource.
    pub fn from_res(
        res: &ResourceLoader,
        path: &Path
    ) -> Result<Shader, Error> {
        const EXT_TYPES: [(&str, gl::types::GLenum); 2] = [
            ("vert", gl::VERTEX_SHADER),
            ("frag", gl::FRAGMENT_SHADER)
        ];

        let name = path.to_str().ok_or(Error::EncodingError)?;
        let path_ext = path.extension().unwrap_or(OsStr::new(""));
        let shader_type = EXT_TYPES.iter()
            .find(|&&(ext, _)| {
                ext == path_ext
            })
            .map(|&(_, s_type)| s_type)
            .ok_or(Error::UnknownShaderType { name: name.into() })?;

        let source = res.load_cstring(path)
            .map_err(|e| Error::ResourceLoadError { name: name.into(), inner: e })?;

        Shader::from_source(&source, shader_type)
            .map_err(|m| Error::CompileError { name: name.into(), message: m})
    }

    /// Load shader source from null-terminated buffer.
    pub fn from_source(
        source: &CStr, // need null-terminated buffer
        shader_type: gl::types::GLuint
    ) -> Result<Shader, String> {
        let id = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        if success == 0 { // error compiling shader
            let error = alloc_whitespace_cstring(len as usize);
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Shader {id} )
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
        unsafe { gl::DeleteShader(self.id); }
    }
}

fn alloc_whitespace_cstring(len: usize) -> CString {
    let buf: Vec<u8> = vec![b' '; len as usize];
    CString::new(buf).unwrap()
}
