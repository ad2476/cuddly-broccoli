//! Enumerate GL constants for better typing.

use gl;

/// Enumerate possible `Glenum` variants for representing triangle layouts.
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

