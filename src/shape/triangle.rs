use gl;
use std::rc::Rc;
use render_gl::types::*;
use render_gl::{self, VBOAttribMarker};
use glm::{self,vec3};

use shape::Drawable;

pub struct Triangle {
    program: Rc<render_gl::Program>,
    _vbo: render_gl::VBO,
    vao: render_gl::VAO,
}

impl Triangle {
    pub fn new(program: &Rc<render_gl::Program>) -> Triangle {
        let vertex_data: Vec<render_gl::Vertex> = vec![
            (vec3(-0.5, -0.5, 0.0), vec3(1.0,0.0,0.0)).into(),
            (vec3(0.5, -0.5, 0.0), vec3(0.0, 1.0, 0.0)).into(),
            (vec3(0.0, 0.5, 0.0), vec3(0.0, 0.0, 1.0)).into()
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
                0),
            VBOAttribMarker::new(
                ShaderAttrib::COLOR,
                VertexAttrib::FLOAT,
                3,
                gl::FALSE,
                ::std::mem::size_of::<glm::Vec3>())
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

