use gl;
use std::rc::Rc;
use render_gl::types::*;
use render_gl::{self, VBOAttribMarker};
use glm::{self,vec3,vec2};

use shape::{Drawable, DrawError};

pub struct Triangle {
    program: Rc<render_gl::Program>,
    _vbo: render_gl::VBO,
    vao: render_gl::VAO,
    time: f32,
}

impl Triangle {
    pub fn new(program: &Rc<render_gl::Program>) -> Triangle {
        let vertex_data: Vec<render_gl::Vertex> = vec![
            (vec3(-0.5, -0.5, 0.0), vec2(0.0, 0.0)).into(),
            (vec3(0.5, -0.5, 0.0), vec2(1.0, 0.0)).into(),
            (vec3(0.0, 0.5, 0.0), vec2(0.5, 1.0)).into()
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
                ShaderAttrib::TEXCOORD0,
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
            vao,
            time: 0.
        }
    }
}

impl Drawable for Triangle {
    fn tick(&mut self) {
        self.time += 0.05;
    }

    fn draw(&self) -> Result<(), DrawError> {
        self.program.bind();
        self.program.set_uniform("u_time", &self.time)?;

        self.vao.bind();
        self.vao.draw();
        self.vao.unbind();

        Ok(())
    }
}

