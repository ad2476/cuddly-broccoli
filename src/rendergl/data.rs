use gl;
use glm;
use crate::rendergl::types;
use crate::util::SurfacePoint;

/// Defines an interface for generic vertex data representations.
///
/// See the implementations of `VertexUV` or `VertexN` for working examples
/// of structs that implement this trait.
pub trait Vertex {
    /// Query attributes for each vertex component.
    ///
    /// Returns a list of attribute markers that determine how OpenGL
    /// should interpret the raw buffer data passed to the shader.
    fn vertex_attrib_markers() -> Vec<VBOAttribMarker>;

    /// Construct an implementing type from a `SurfacePoint`.
    ///
    /// For example, given a `SurfacePoint`, this might involve querying methods such as
    /// `SurfacePoint::position()` or `SurfacePoint::normal()` to construct
    /// a `VertexN`.
    fn from_point3d(point: &SurfacePoint) -> Self;
}

/// Representation of a vertex with position.
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertexP {
    pos: glm::Vec3,
}
impl VertexP {
    pub fn new(pos: glm::Vec3) -> VertexP {
        VertexP { pos }
    }
}
impl Vertex for VertexP {
    fn vertex_attrib_markers() -> Vec<VBOAttribMarker> {
        let markers: Vec<VBOAttribMarker> = vec![VBOAttribMarker::new(
            types::ShaderAttrib::POSITION,
            types::VertexAttrib::FLOAT,
            3,
            gl::FALSE,
            0,
        )];
        markers
    }

    fn from_point3d(point: &SurfacePoint) -> VertexP {
        VertexP::new(point.position())
    }
}
impl From<glm::Vec3> for VertexP {
    fn from(other: glm::Vec3) -> VertexP {
        VertexP::new(other)
    }
}

/// Representation of a vertex with position and uv coordinates.
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertexUV {
    pos: glm::Vec3,
    uv: glm::Vec2,
}
impl VertexUV {
    pub fn new(pos: glm::Vec3, uv: glm::Vec2) -> VertexUV {
        VertexUV { pos, uv }
    }
}
impl Vertex for VertexUV {
    fn vertex_attrib_markers() -> Vec<VBOAttribMarker> {
        let markers: Vec<VBOAttribMarker> = vec![
            VBOAttribMarker::new(
                types::ShaderAttrib::POSITION,
                types::VertexAttrib::FLOAT,
                3,
                gl::FALSE,
                0,
            ),
            VBOAttribMarker::new(
                types::ShaderAttrib::TEXCOORD0,
                types::VertexAttrib::FLOAT,
                2,
                gl::FALSE,
                ::std::mem::size_of::<glm::Vec3>(),
            ),
        ];
        markers
    }

    fn from_point3d(point: &SurfacePoint) -> VertexUV {
        VertexUV::new(point.position(), point.texcoord())
    }
}
impl From<(glm::Vec3, glm::Vec2)> for VertexUV {
    fn from(other: (glm::Vec3, glm::Vec2)) -> VertexUV {
        VertexUV::new(other.0, other.1)
    }
}

/// Representation of a vertex with position, normal, and texture coordinates.
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertexNT {
    pos: glm::Vec3,
    n: glm::Vec3,
    uv: glm::Vec2,
}
impl VertexNT {
    pub fn new(pos: glm::Vec3, normal: glm::Vec3, uv: glm::Vec2) -> VertexNT {
        VertexNT { pos, n: normal, uv }
    }
}
impl Vertex for VertexNT {
    fn vertex_attrib_markers() -> Vec<VBOAttribMarker> {
        let markers: Vec<VBOAttribMarker> = vec![
            VBOAttribMarker::new(
                types::ShaderAttrib::POSITION,
                types::VertexAttrib::FLOAT,
                3,
                gl::FALSE,
                0,
            ),
            VBOAttribMarker::new(
                types::ShaderAttrib::NORMAL,
                types::VertexAttrib::FLOAT,
                3,
                gl::FALSE,
                ::std::mem::size_of::<glm::Vec3>(),
            ),
            VBOAttribMarker::new(
                types::ShaderAttrib::TEXCOORD0,
                types::VertexAttrib::FLOAT,
                2,
                gl::FALSE,
                ::std::mem::size_of::<glm::Vec3>() * 2,
            ),
        ];
        markers
    }

    fn from_point3d(point: &SurfacePoint) -> VertexNT {
        VertexNT::new(point.position(), point.normal(), point.texcoord())
    }
}
impl From<(glm::Vec3, glm::Vec3, glm::Vec2)> for VertexNT {
    fn from(other: (glm::Vec3, glm::Vec3, glm::Vec2)) -> VertexNT {
        VertexNT::new(other.0, other.1, other.2)
    }
}

/// Representation of a vertex with position and normal.
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertexN {
    pos: glm::Vec3,
    n: glm::Vec3,
}
impl VertexN {
    pub fn new(pos: glm::Vec3, normal: glm::Vec3) -> VertexN {
        VertexN { pos, n: normal }
    }
}
impl Vertex for VertexN {
    fn vertex_attrib_markers() -> Vec<VBOAttribMarker> {
        let markers: Vec<VBOAttribMarker> = vec![
            VBOAttribMarker::new(
                types::ShaderAttrib::POSITION,
                types::VertexAttrib::FLOAT,
                3,
                gl::FALSE,
                0,
            ),
            VBOAttribMarker::new(
                types::ShaderAttrib::NORMAL,
                types::VertexAttrib::FLOAT,
                3,
                gl::FALSE,
                ::std::mem::size_of::<glm::Vec3>(),
            ),
        ];
        markers
    }

    fn from_point3d(point: &SurfacePoint) -> VertexN {
        VertexN::new(point.position(), point.normal())
    }
}
impl From<(glm::Vec3, glm::Vec3)> for VertexN {
    fn from(other: (glm::Vec3, glm::Vec3)) -> VertexN {
        VertexN::new(other.0, other.1)
    }
}

/// Mark a specific VBO attribute (such as position, color, etc)
/// for passing to
/// [`glVertexAttribPointer`](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointer.xhtml).
pub struct VBOAttribMarker {
    pub name: types::ShaderAttrib,      // attribute location
    pub data_type: types::VertexAttrib, // primitive type in VBO
    pub elements_per_vertex: gl::types::GLint,
    pub normalize: gl::types::GLboolean, // normalise data
    pub offset: usize,                   // offset in bytes from start of array to first element
}

impl VBOAttribMarker {
    pub fn new(
        name: types::ShaderAttrib,
        data_type: types::VertexAttrib,
        elements_per_vertex: gl::types::GLint,
        normalize: gl::types::GLboolean,
        offset: usize,
    ) -> VBOAttribMarker {
        VBOAttribMarker {
            name,
            data_type,
            elements_per_vertex,
            normalize,
            offset,
        }
    }
}
