use std;
use gl;
use glm;
use render_gl::types;

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

/// Mark a specific VBO attribute (such as position, color, etc)
/// for passing to `glVertexAttribPointer`.
pub struct VBOAttribMarker {
    name: types::ShaderAttrib, // attribute location
    data_type: types::VertexAttrib, // primitive type in VBO
    elements_per_vertex: gl::types::GLint,
    normalise: gl::types::GLboolean, // normalise data
    offset: usize, // offset in bytes from start of array to first element
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

/// Vertex Buffer Object.
///
/// Passes an array of vertex data to GPU and wraps binding and cleanup.
pub struct VBO {
    id: gl::types::GLuint,
    markers: Vec<VBOAttribMarker>,
    buffer_size: usize, // number of vertices
//    f32_per_vert: usize, // elements per vertex
    stride: gl::types::GLint,
}

impl VBO {
    pub fn from_data<T: Vertex>(data: &[T]) -> VBO
    {
        let mut id: gl::types::GLuint = 0;
        let buffer_size = data.len();
        let stride = std::mem::size_of::<T>();
        let markers = T::vertex_attrib_markers();
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id); // bind handle
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (buffer_size * stride) as gl::types::GLsizeiptr, // size in bytes
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind
        }
        VBO { id, markers, buffer_size, stride: stride as i32 }
    }

    // FIXME this bind-unbind semantics feels unsafe/not rusty
    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id); }
    }

    /// Enable this vertex array. VBO **must be bound**.
    fn enable(&self) {
        for m in &self.markers {
            unsafe {
                gl::EnableVertexAttribArray(m.name.into());
                gl::VertexAttribPointer(
                    m.name.into(),
                    m.elements_per_vertex,
                    m.data_type.into(),
                    m.normalise,
                    self.stride,
                    m.offset as *const gl::types::GLvoid
                );
            }
        }
    }

    fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0); }
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}

/// Vertex Attribute Object.
///
/// Associates attributes with a VBO and is responsible
/// for drawing the vertex buffer.
pub struct VAO {
    id: gl::types::GLuint,
    num_vertices: gl::types::GLsizei,
    layout: types::GlLayout,
}

impl VAO {
    pub fn new(
        vbo: &VBO,
        layout: types::GlLayout
    ) -> VAO
    {
        let mut id: gl::types::GLuint = 0;
        let num_vertices = vbo.buffer_size as gl::types::GLsizei;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
        }
        vbo.bind();
        vbo.enable();
        vbo.unbind();
        unsafe { gl::BindVertexArray(0); }
        VAO { id, num_vertices, layout }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id); }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawArrays(self.layout.into(), 0, self.num_vertices);
        }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0); }
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id); }
    }
}
