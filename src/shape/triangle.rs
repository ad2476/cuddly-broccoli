use std::rc::Rc;
use rendergl::types::*;
use rendergl;
use glm::{self,vec3,vec2};
use num;

use shape::{Drawable, DrawError};

pub struct Triangle {
    program: Rc<rendergl::Program>,
    _vbo: rendergl::VBO,
    ibo: rendergl::IBO,
    vao: rendergl::VAO,
    transform: glm::Mat4,
    time: f32,
}

impl Triangle {
    pub fn new(program: &Rc<rendergl::Program>) -> Triangle {
        let vertex_data: Vec<rendergl::VertexUV> = vec![
            (vec3(-0.5, -0.5, 0.0), vec2(0.0, 0.0)).into(),
            (vec3(0.5, -0.5, 0.0), vec2(1.0, 0.0)).into(),
            (vec3(0.0, 0.5, 0.0), vec2(0.5, 1.0)).into(),
            (vec3(0.5, 0.5, 0.0), vec2(1.0, 1.0)).into()
        ];
        Triangle::from_data(vertex_data, program)
    }

    pub fn from_data(
        data: Vec<rendergl::VertexUV>,
        program: &Rc<rendergl::Program>
    ) -> Triangle {
        let index_data: Vec<u32> = vec![0, 1, 2, 2, 1, 3];

        let vbo = rendergl::VBO::from_data(&data);
        let ibo = rendergl::IBO::from_data(&index_data);
        let vao = rendergl::VAO::new(
            &vbo,
            Some(&ibo),
            GlLayout::Triangles
        );

        Triangle {
            program: Rc::clone(program),
            _vbo: vbo,
            ibo,
            vao,
            transform: glm::ext::rotate(&num::one(), -glm::ext::consts::half_pi::<f32,f32>(),vec3(0.0,0.0,1.0)),
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
        self.ibo.bind();
        self.vao.draw();
        self.ibo.unbind();
        self.vao.unbind();

        Ok(())
    }
}

