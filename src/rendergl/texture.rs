//! Abstractions on OpenGL textures.
//!
//! # Example
//!
//! Load an image into a `GL_TEXTURE_2D` target, apply paramters, bind the texture:
//! ```
//! let img: &DynamicImage; // assume an image somewhere
//! let texture = Texture::from_image(img);
//! TextureParameters::new()
//!     .wrap_method(WrapMethod::ClampToEdge)
//!     .filter_method(FilterMethod::Linear)
//!     .apply_to(&texture)
//!
//! // we'd probably want to associate our texture with vertex data somehow
//! // (maybe a buffer of VertexUV)
//! texture.bind();
//! // draw vertices + use sampler2D in shader
//! texture.unbind();
//! ```

use gl;
use image::{GenericImageView, DynamicImage};
use rendergl::types::{TextureTarget, TextureParam};

/// Enumerate valid `GLenum` variants for `gl::TEXTURE_*_FILTER` parameters.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FilterMethod {
    Nearest = gl::NEAREST,
    Linear = gl::LINEAR,
}
impl From<FilterMethod> for gl::types::GLenum {
    fn from(item: FilterMethod) -> gl::types::GLenum {
        item as gl::types::GLenum
    }
}

/// Enumerate valid `GLenum` variants for `gl::TEXTURE_WRAP_*` parameters.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum WrapMethod {
    ClampToEdge = gl::CLAMP_TO_EDGE,
    ClampToBorder = gl::CLAMP_TO_BORDER,
    Repeat = gl::REPEAT,
    MirroredRepeat = gl::MIRRORED_REPEAT,
    MirrorClampToEdge = gl::MIRROR_CLAMP_TO_EDGE,
}
impl From<WrapMethod> for gl::types::GLenum {
    fn from(item: WrapMethod) -> gl::types::GLenum {
        item as gl::types::GLenum
    }
}

type ParamPair = (TextureParam, gl::types::GLenum);

/// Holds a list of GL texture parameters to apply with `glTexParameteri`.
///
/// # Usage
///
/// Apply some `TextureParameters` to a `Texture`:
/// ```
/// let img: &DynamicImage; // texture image
/// let texture = Texture::from_image(img);
/// TextureParameters::new()
///     .wrap_method(WrapMethod::ClampToEdge)
///     .filter_method(FilterMethod::Linear)
///     .apply_to(&texture)
/// ```
pub struct TextureParameters {
    params: Vec<ParamPair>,
}

impl TextureParameters {

    /// Construct a new `TextureParameters` instance.
    pub fn new() -> TextureParameters {
        TextureParameters {
            params: Vec::new(),
        }
    }

    /// Set a value for a parameter to `glTexParameteri`.
    pub fn set_param(&mut self, param: TextureParam, value: gl::types::GLenum) -> &mut TextureParameters {
        self.params.push((param, value));
        self
    }

    /// Set the filter method used by `GL_TEXTURE_*_FILTER`.
    pub fn filter_method(&mut self, method: FilterMethod) -> &mut TextureParameters {
        self.params.push((TextureParam::MinFilter, method.into()));
        self.params.push((TextureParam::MagFilter, method.into()));
        self
    }

    /// Set the wrap method used by `GL_TEXTURE_WRAP_S` and `GL_TEXTURE_WRAP_T`.
    pub fn wrap_method2d(&mut self, method: WrapMethod) -> &mut TextureParameters {
        self.params.push((TextureParam::WrapS, method.into()));
        self.params.push((TextureParam::WrapT, method.into()));
        self
    }

    /// Set the wrap methods used by `GL_TEXTURE_WRAP_S`, `GL_TEXTURE_WRAP_T` and `GL_TEXTURE_WRAP_R`.
    pub fn wrap_method3d(&mut self, method: WrapMethod) -> &mut TextureParameters {
        self.wrap_method2d(method)
            .set_param(TextureParam::WrapR, method.into())
    }

    /// Apply these parameters onto a `Texture`.
    pub fn apply_to(&self, tex: &Texture) {
        tex.bind();
        for p in &self.params {
            unsafe {
                gl::TexParameteri(tex.target.into(), p.0.into(), p.1 as gl::types::GLint);
            }
        }
        tex.unbind();
    }
}

/// Texture object.
///
/// Wrapper type for OpenGL textures.
pub struct Texture {
    /// Returns this texture's handle generated by `glGenTextures`.
    pub id: gl::types::GLuint,
    pub target: TextureTarget,
}

impl Texture {
    const INTERNAL_FORMAT: gl::types::GLint = gl::RGBA8 as gl::types::GLint;
    const PIXEL_TYPE: gl::types::GLenum = gl::UNSIGNED_BYTE;

    fn gen_handle() -> gl::types::GLuint {
        let mut id: gl::types::GLuint = 0;
        unsafe { // generate a texture handle
            gl::GenTextures(1, &mut id);
        }
        id
    }

    /// Selects the currently active texture unit.
    ///
    /// See the OpenGL docs on [glActiveTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveTexture.xhtml).
    pub fn active_texture(unit: gl::types::GLenum) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
        }
    }

    /// Wrapper for `glTexImage2D` that takes a reference to a `DynamicImage`.
    ///
    /// Texture handle must be bound before calling.
    fn tex_image_2d(target: gl::types::GLenum, img: &DynamicImage) {
        let level: gl::types::GLint = 0;
        let (width, height) = img.dimensions();
        let pixels = img.raw_pixels().as_ptr() as *const gl::types::GLvoid;
        let format = image_gl_format(&img);
        unsafe {
            gl::TexImage2D(
                target, // texture target
                level, // mipmap level
                Texture::INTERNAL_FORMAT,
                width as i32,
                height as i32,
                0, // must be 0 (OpenGL....)
                format,
                Texture::PIXEL_TYPE, // data type of pixel data
                pixels);
        }
    }

    /// Construct a new `Texture` with `Tex2D` target and a handle.
    fn texture_2d() -> Texture {
        Texture {
            id: Texture::gen_handle(),
            target: TextureTarget::Tex2D,
        }
    }

    /// Construct a new `Texture` with `Tex2D` target and a handle.
    fn texture_cubemap() -> Texture {
        Texture {
            id: Texture::gen_handle(),
            target: TextureTarget::TexCubeMap,
        }
    }

    /// Initialise a `Texture2D` from an image.
    pub fn from_image(img: &DynamicImage) -> Texture {
        let tex = Texture::texture_2d();

        tex.bind();
        Texture::tex_image_2d(tex.target.into(), img);
        tex.unbind();

        tex
    }

    /// Construct a cubemap from a collection of 6 `DynamicImage`s.
    ///
    /// Returns:
    ///
    /// * `Some(Texture)` if `faces` has 6 elements
    /// * `None` otherwise.
    pub fn cubemap(faces: &[DynamicImage]) -> Option<Texture> {
        if faces.len() != 6 {
            return None
        }

        let tex = Texture::texture_cubemap();

        tex.bind();
        for (i, img) in faces.iter().enumerate() {
            let i = i as u32;
            Texture::tex_image_2d(gl::TEXTURE_CUBE_MAP_POSITIVE_X + i, img);
        }
        tex.unbind();

        Some(tex)
    }

    /// Generate mipmaps for this texture's target type.
    /// Texture must be bound.
    ///
    /// See [glGenerateMipmap](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenerateMipmap.xhtml).
    pub fn generate_mipmap(&self) {
        unsafe {
            gl::GenerateMipmap(self.target.into());
        }
    }

    /// Binds the texture.
    ///
    /// See [glBindTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTexture.xhtml).
    pub fn bind(&self) {
        unsafe { gl::BindTexture(self.target.into(), self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindTexture(self.target.into(), 0); }
    }

}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

fn image_gl_format(img: &DynamicImage) -> gl::types::GLenum {
    match img {
        DynamicImage::ImageLuma8(_) => gl::RED,
        DynamicImage::ImageLumaA8(_) => gl::RG,
        DynamicImage::ImageRgb8(_) => gl::RGB,
        DynamicImage::ImageRgba8(_) => gl::RGBA,
        DynamicImage::ImageBgr8(_) => gl::BGR,
        DynamicImage::ImageBgra8(_) => gl::BGRA,
    }
}
