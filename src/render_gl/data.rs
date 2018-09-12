use std;
use gl;
use glm;
use render_gl::types;

/// Representation of a Vertex in the shader. For now, let this be a struct,
/// but it would be nice to have it as a Trait and expose ways of getting at
/// vertex attributes
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pos: glm::Vec3,
    uv: glm::Vec2,
}

impl Vertex {
    pub fn new(pos: glm::Vec3, uv: glm::Vec2) -> Vertex {
        Vertex { pos, uv }
    }
}

impl From<(glm::Vec3, glm::Vec2)> for Vertex {
    fn from(other: (glm::Vec3, glm::Vec2)) -> Vertex {
        Vertex::new(other.0, other.1)
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
    pub fn from_data(
        data: &Vec<Vertex>,
        markers: Vec<VBOAttribMarker>,
    ) -> VBO
    {
        let mut id: gl::types::GLuint = 0;
        let buffer_size = data.len();
        let stride = std::mem::size_of::<Vertex>();
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
