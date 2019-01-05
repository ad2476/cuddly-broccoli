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
    pub fn wrap_method(&mut self, method: WrapMethod) -> &mut TextureParameters {
        self.params.push((TextureParam::WrapS, method.into()));
        self.params.push((TextureParam::WrapT, method.into()));
        self
    }

    /// Apply these parameters onto a `Texture`.
    pub fn apply_to(&self, tex: &Texture) {
        tex.bind();
        for p in &self.params {
            unsafe {
                gl::TexParameteri(tex.target().into(), p.0.into(), p.1 as gl::types::GLint);
            }
        }
        tex.unbind();
    }
}

/// Texture object.
///
/// Wrapper type for OpenGL textures.
pub enum Texture {
    Texture2D(gl::types::GLuint),
    // TODO: enumerate more texture targets
}

impl Texture {
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

    /// Initialise a `Texture2D` from an image.
    pub fn from_image(img: &DynamicImage) -> Texture {
        // TODO: what happens for other texture types?
        let tex = Texture::Texture2D(Texture::gen_handle());

        let target = tex.target();
        let level: gl::types::GLint = 0;
        let internal_format = gl::RGBA as gl::types::GLint;
        let (width, height) = img.dimensions();
        let type_ = gl::UNSIGNED_BYTE;
        let pixels = img.raw_pixels().as_ptr() as *const gl::types::GLvoid;
        let format = image_gl_format(&img);

        tex.bind();
        unsafe {
            gl::TexImage2D(target.into(), level, internal_format, width as i32, height as i32, 0, format, type_, pixels);
        }
        tex.unbind();

        tex
    }

    /// Based on the enum type, returns active target.
    pub fn target(&self) -> TextureTarget {
        match self {
            Texture::Texture2D(_) => TextureTarget::Tex2D,
        }
    }

    /// Generate mipmaps for this texture's target type.
    /// Texture must be bound.
    ///
    /// See [glGenerateMipmap](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenerateMipmap.xhtml).
    pub fn generate_mipmap(&self) {
        unsafe {
            gl::GenerateMipmap(self.target().into());
        }
    }

    /// Binds the texture.
    ///
    /// See [glBindTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTexture.xhtml).
    pub fn bind(&self) {
        unsafe { gl::BindTexture(self.target().into(), self.id()); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindTexture(self.target().into(), 0); }
    }

    /// Returns this texture's handle generated by `glGenTextures`.
    pub fn id(&self) -> gl::types::GLuint {
        match self {
            Texture::Texture2D(id) => *id,
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id());
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
