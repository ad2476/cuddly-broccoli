use std::rc::Rc;
use render_gl::types::*;
use render_gl;
use glm::{self,vec3,vec2};
use num;

use shape::{Drawable, DrawError};

pub struct Triangle {
    program: Rc<render_gl::Program>,
    _vbo: render_gl::VBO,
    vao: render_gl::VAO,
    transform: glm::Mat4,
    time: f32,
}

impl Triangle {
    pub fn new(program: &Rc<render_gl::Program>) -> Triangle {
        let vertex_data: Vec<render_gl::VertexUV> = vec![
            (vec3(-0.5, -0.5, 0.0), vec2(0.0, 0.0)).into(),
            (vec3(0.5, -0.5, 0.0), vec2(1.0, 0.0)).into(),
            (vec3(0.0, 0.5, 0.0), vec2(0.5, 1.0)).into()
        ];
        Triangle::from_data(vertex_data, program)
    }

    pub fn from_data(
        data: Vec<render_gl::VertexUV>,
        program: &Rc<render_gl::Program>
    ) -> Triangle {
        let vbo = render_gl::VBO::from_data(&data);
        let vao = render_gl::VAO::new(
            &vbo,
            GlLayout::Triangles
        );

        Triangle {
            program: Rc::clone(program),
            _vbo: vbo,
            vao,
            transform: glm::ext::translate(&num::one(), vec3(-0.5,0.0,0.0)),
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
        self.program.set_uniform("m", &self.transform)?;

        self.vao.bind();
        self.vao.draw();
        self.vao.unbind();

        Ok(())
    }
}

