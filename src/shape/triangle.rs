use gl;
use std::rc::Rc;
use render_gl::types::*;
use render_gl::{self, VBOAttribMarker};

use shape::Drawable;

pub struct Triangle {
    program: Rc<render_gl::Program>,
    _vbo: render_gl::VBO,
    vao: render_gl::VAO,
}

impl Triangle {
    pub fn new(program: &Rc<render_gl::Program>) -> Triangle {
        let vertex_data: Vec<render_gl::Vertex> = vec![
            (-0.5, -0.5, 0.0).into(),
            (0.5, -0.5, 0.0).into(),
            (0.0, 0.5, 0.0).into()
        ];
        Triangle::from_data(vertex_data, program)
    }

    pub fn from_data(
        data: Vec<render_gl::Vertex>,
        program: &Rc<render_gl::Program>
    ) -> Triangle {
        let markers: Vec<VBOAttribMarker> = vec![
            VBOAttribMarker::new(
                ShaderAttrib::POSITION, 
                VertexAttrib::FLOAT,
                3,
                gl::FALSE,
                0)
        ];
        let vbo = render_gl::VBO::from_data(
            &data,
            markers
        );
        let vao = render_gl::VAO::new(
            &vbo,
            GlLayout::Triangles
        );

        Triangle {
            program: Rc::clone(program),
            _vbo: vbo,
            vao
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self) {
        self.program.set_used();
        self.vao.bind();
        self.vao.draw();
        self.vao.unbind();
    }
}

