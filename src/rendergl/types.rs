//! Enumerate GL constants for better typing.

use gl;

/// Enumerate possible `GLenum` variants for texture parameter names.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TextureParam {
    DepthStencilMode = gl::DEPTH_STENCIL_TEXTURE_MODE,
    BaseLevel = gl::TEXTURE_BASE_LEVEL,
    CompareFunction = gl::TEXTURE_COMPARE_FUNC,
    CompareMode = gl::TEXTURE_COMPARE_MODE,
    LodBias = gl::TEXTURE_LOD_BIAS,
    MinFilter = gl::TEXTURE_MIN_FILTER,
    MagFilter = gl::TEXTURE_MAG_FILTER,
    MinLod = gl::TEXTURE_MIN_LOD,
    MaxLod = gl::TEXTURE_MAX_LOD,
    MaxLevel = gl::TEXTURE_MAX_LEVEL,
    SwizzleR = gl::TEXTURE_SWIZZLE_R,
    SwizzleG = gl::TEXTURE_SWIZZLE_G,
    SwizzleB = gl::TEXTURE_SWIZZLE_B,
    SwizzleA = gl::TEXTURE_SWIZZLE_A,
    WrapS = gl::TEXTURE_WRAP_S,
    WrapT = gl::TEXTURE_WRAP_T,
    WrapR = gl::TEXTURE_WRAP_R,

    // For the vector commands glTexParameter*v, pname can be one of:
    BorderColor = gl::TEXTURE_BORDER_COLOR,
    SwizzleRGBA = gl::TEXTURE_SWIZZLE_RGBA,
}
impl From<TextureParam> for gl::types::GLenum {
    fn from(item: TextureParam) -> gl::types::GLenum {
        item as gl::types::GLenum
    }
}

/// Enumerate possible `GLenum` variants for representing texture targets.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TextureTarget {
    Tex1D = gl::TEXTURE_1D,
    Tex1DArray = gl::TEXTURE_1D_ARRAY,
    Tex2D = gl::TEXTURE_2D,
    Tex2DArray = gl::TEXTURE_2D_ARRAY,
    Tex2DMultisample = gl::TEXTURE_2D_MULTISAMPLE,
    Tex2DMultisampleArray = gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
    Tex3D = gl::TEXTURE_3D,
    TexCubeMap = gl::TEXTURE_CUBE_MAP,
    TexCubeMapArray = gl::TEXTURE_CUBE_MAP_ARRAY,
    TexRectangle = gl::TEXTURE_RECTANGLE,
}
impl From<TextureTarget> for gl::types::GLenum {
    fn from(item: TextureTarget) -> gl::types::GLenum {
        item as gl::types::GLenum
    }
}

/// Enumerate possible `GLenum` variants for representing triangle layouts.
///
/// Refer to: [OpenGL docs](https://www.khronos.org/opengl/wiki/Primitive#Triangle_primitives)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum GlLayout {
    Triangles = gl::TRIANGLES,
    TriangleStrip = gl::TRIANGLE_STRIP,
    TriangleFan = gl::TRIANGLE_FAN,
    LineStrip = gl::LINE_STRIP,
}
impl From<GlLayout> for gl::types::GLenum {
    fn from(item: GlLayout) -> gl::types::GLenum {
        item as gl::types::GLenum
    }
}

/// Enumerate shader locations.
///
/// When writing vertex shaders, layout must match. For example:
///
/// ```c,ignore
/// layout(location = 0) in vec3 position;
/// layout(location = 5) in vec2 texcoord0;
/// ```
///
/// # Attrib locations
///
/// | Name | Location |
/// | ---- | :------: |
/// | `POSITION`  | 0 |
/// | `NORMAL`    | 1 |
/// | `COLOR`     | 2 |
/// | `TANGENT`   | 3 |
/// | `BINORMAL`  | 4 |
/// | `TEXCOORD0` | 5 |
/// | `TEXCOORD1` | 6 |
/// | `TEXCOORD2` | 7 |
/// | `TEXCOORD3` | 8 |
/// | `SPECIAL0`  | 9 |
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum ShaderAttrib {
    POSITION = 0,
    NORMAL,
    COLOR,
    TANGENT,
    BINORMAL,
    TEXCOORD0,
    TEXCOORD1,
    TEXCOORD2,
    TEXCOORD3,
    SPECIAL0,
}
impl From<ShaderAttrib> for gl::types::GLuint {
    fn from(item: ShaderAttrib) -> gl::types::GLuint {
        item as gl::types::GLuint
    }
}

/// Enumerate accepted types for
/// [`glVertexAttribPointer`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointer.xhtml).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VertexAttrib {
    BYTE = gl::BYTE,
    UBYTE = gl::UNSIGNED_BYTE,
    SHORT = gl::SHORT,
    USHORT = gl::UNSIGNED_SHORT,
    INT = gl::INT,
    UINT = gl::UNSIGNED_INT,
    FLOAT = gl::FLOAT,
}
impl From<VertexAttrib> for gl::types::GLenum {
    fn from(item: VertexAttrib) -> gl::types::GLenum {
        item as gl::types::GLenum
    }
}
