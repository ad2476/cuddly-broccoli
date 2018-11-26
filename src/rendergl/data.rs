use gl;
use glm;
use rendergl::types;

/// Trait for data that represents a vertex to implement. Defines an
/// interface for getting attribute markers on the vertex data, so
/// that OpenGL knows what to pass to the shader.
pub trait Vertex {
    fn vertex_attrib_markers() -> Vec<VBOAttribMarker>;
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
                0),
            VBOAttribMarker::new(
                types::ShaderAttrib::TEXCOORD0,
                types::VertexAttrib::FLOAT,
                2,
                gl::FALSE,
                ::std::mem::size_of::<glm::Vec3>())
        ];
        markers
    }
}
impl From<(glm::Vec3, glm::Vec2)> for VertexUV {
    fn from(other: (glm::Vec3, glm::Vec2)) -> VertexUV {
        VertexUV::new(other.0, other.1)
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
                0),
            VBOAttribMarker::new(
                types::ShaderAttrib::NORMAL,
                types::VertexAttrib::FLOAT,
                3,
                gl::FALSE,
                ::std::mem::size_of::<glm::Vec3>())
        ];
        markers
    }
}
impl From<(glm::Vec3, glm::Vec3)> for VertexN {
    fn from(other: (glm::Vec3, glm::Vec3)) -> VertexN {
        VertexN::new(other.0, other.1)
    }
}

/// Mark a specific VBO attribute (such as position, color, etc)
/// for passing to `glVertexAttribPointer`.
pub struct VBOAttribMarker {
    pub name: types::ShaderAttrib, // attribute location
    pub data_type: types::VertexAttrib, // primitive type in VBO
    pub elements_per_vertex: gl::types::GLint,
    pub normalise: gl::types::GLboolean, // normalise data
    pub offset: usize, // offset in bytes from start of array to first element
}

impl VBOAttribMarker {
    pub fn new(
        name: types::ShaderAttrib,
        data_type: types::VertexAttrib,
        elements_per_vertex: gl::types::GLint,
        normalise: gl::types::GLboolean,
        offset: usize
    ) -> VBOAttribMarker
    {
        VBOAttribMarker {
            name,
            data_type,
            elements_per_vertex,
            normalise,
            offset
        }
    }
}

