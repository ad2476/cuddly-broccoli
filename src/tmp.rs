/// Representation of a Vertex in the shader. For now, let this be a struct,
/// but it would be nice to have it as a Trait and expose ways of getting at
/// vertex attributes
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pos: glm::Vec3,
    col: glm::Vec3,
}

impl Vertex {
    pub fn len(&self) -> gl::types::GLuint {
        6
    }
}

/// Mark a specific VBO attribute (such as position, color, etc)
/// for passing to `glVertexAttribPointer`.
pub struct VBOAttribMarker {
    name: ShaderAttrib, // attribute location
    data_type: VertexAttribType, // primitive type in VBO
    elements_per_vertex: gl::types::GLuint,
    normalise: gl::types::GLboolean, // normalise data
    offset: usize, // offset in bytes from start of array to first element
}

/// Vertex Buffer Object.
///
/// Passes an array of vertex data to GPU and wraps binding and cleanup.
pub struct VBO {
    id: gl::types::GLuint,
    Vec<VBOAttribMarker> markers,
    buffer_size: usize, // number of floats in buffer
//    f32_per_vert: usize, // elements per vertex
    stride: gl::types::GLuint,
    layout: GlLayout,
}

impl VBO {
    pub fn from_data(
        data: &Vec<Vertex>,
        markers: Vec<VBOAttribMarker>,
        layout: GlLayout)
    {
        let mut id: gl::types::GLuint = 0;
        let buffer_size = data.len();
        let stride = std::mem::size_of::<Vertex>();
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id); // bind handle
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (buffer_size * stride) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind
        }
        VBO { id, markers, buffer_size, stride, layout }
    }

    // FIXME this bind-unbind semantics feels unsafe/not rusty
    pub fn bind(&self) {
        unsafe { gl::BindBufffer(gl::ARRAY_BUFFER, self.id); }
    }

    /// Enable this vertex array. VBO **must be bound**.
    pub fn enable(&self) {
        for m in self.markers {
            unsafe {
                gl::EnableVertexAttribArray(m.name);
                gl::VertexAttribPointer(
                    m.name,
                    m.elements_per_vertex,
                    m.data_type,
                    m.normalise,
                    self.stride,
                    m.offset as *const gl::types::GLvoid
                );
            }
        }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0); }
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}

