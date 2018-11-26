use std;
use gl;
use rendergl::{self,types};

/// Vertex Buffer Object.
///
/// Passes an array of vertex data to GPU and wraps binding and cleanup.
pub struct VBO {
    id: gl::types::GLuint,
    markers: Vec<rendergl::VBOAttribMarker>,
    buffer_size: usize, // number of vertices
//    f32_per_vert: usize, // elements per vertex
    stride: gl::types::GLint,
}

impl VBO {
    pub fn from_data<T: rendergl::Vertex>(data: &[T]) -> VBO
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

/// Index Buffer Object.
///
/// Defines a permutation of indices over an associated VBO, and can be used
/// for indexed drawing.
pub struct IBO {
    id: gl::types::GLuint,
    buffer_size: usize,
}

impl IBO {
    pub fn from_data(data: &[u32]) -> IBO {
        let mut id: gl::types::GLuint = 0;
        let buffer_size = data.len();
        let stride = std::mem::size_of::<u32>();
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id); // bind handle
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (buffer_size * stride) as gl::types::GLsizeiptr, // size in bytes
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); // unbind
        }
        IBO { id, buffer_size }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); }
    }
}

impl Drop for IBO {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}

/// Enumerate draw methods for VAOs
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum DrawMethod {
    ARRAYS,
    INDEXED,
}

/// Vertex Array Object.
///
/// Associates attributes with a VBO and is responsible
/// for drawing the vertex buffer.
pub struct VAO {
    id: gl::types::GLuint,
    num_vertices: gl::types::GLsizei, // number of vertices to render
    layout: types::GlLayout,
    draw_method: DrawMethod,
}

impl VAO {
    pub fn new(
        vbo: &VBO,
        ibo: Option<&IBO>,
        layout: types::GlLayout
    ) -> VAO
    {
        let mut id: gl::types::GLuint = 0;
        // number of vertices to render is either the vertices in the VBO, or the indices
        //  in the IBO
        let num_vertices = ibo
            .map_or(vbo.buffer_size, |i| i.buffer_size) as gl::types::GLsizei;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
        }
        vbo.bind();
        let draw_method = match ibo {
            Some(i) => {
                i.bind(); // associate IBO with this VAO
                DrawMethod::INDEXED
            },
            None => DrawMethod::ARRAYS,
        };
        vbo.enable();
        vbo.unbind();
        ibo.map_or((), |i| i.unbind());
        unsafe { gl::BindVertexArray(0); }
        VAO { id, num_vertices, layout, draw_method }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id); }
    }

    pub fn draw(&self) {
        match self.draw_method {
            DrawMethod::ARRAYS => unsafe {
                gl::DrawArrays(self.layout.into(), 0, self.num_vertices);
            },
            DrawMethod::INDEXED => unsafe {
                gl::DrawElements(self.layout.into(),
                                 self.num_vertices,
                                 gl::UNSIGNED_INT,
                                 0 as *const gl::types::GLvoid);
            },
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
