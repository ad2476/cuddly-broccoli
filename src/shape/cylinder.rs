use std::rc::Rc;
use rendergl::types::*;
use rendergl;
use glm::{self,vec3,vec2};
use glm::ext::consts;
use num;

use shape::{Drawable, DrawError};

pub struct Cylinder {
    program: Rc<rendergl::Program>,
    _vbo: rendergl::VBO,
    ibo: rendergl::IBO,
    vao: rendergl::VAO,
    transform: glm::Mat4,
}

impl Cylinder {
    pub fn new(program: &Rc<rendergl::Program>, strips: u32, slices: u32) -> Cylinder {
        let mut vert_data: Vec<rendergl::VertexN> = Vec::new();
        let mut index_data: Vec<u32> = Vec::new();

        const R: f32 = 0.5;
        let pi: f32 = consts::pi();

        let theta_stepsz: f32 = 2.0*pi/(slices as f32);
//        let lat_stepsz: f32 = pi/(strips as f32);
        let r_stepsz: f32 = R/(strips as f32);
        let side_stepsz: f32 = 2.0*R/(strips as f32);

        // generate vertices
        for theta_step in 0..slices {
            let theta = -theta_stepsz*(theta_step as f32);
            let mut v_count = 0;

            // top cap slice
            for r_step in 0..(strips+1) {
                let r = r_stepsz*(r_step as f32);
                let v = vec3(polar_x(r, theta), R, polar_y(r, theta));
                let n = vec3(0.0, 1.0, 0.0);
                vert_data.push((v,n).into());
                v_count += 1;
            }
            println!("(0) v_count: {}", v_count);

            // slice side
            let x_side = polar_x(R, theta);
            let z_side = polar_y(R, theta);
            for y_step in 0..(strips+1) {
                let y = R - side_stepsz*(y_step as f32);
                let v = vec3(x_side, y, z_side);
                let n = glm::normalize(vec2(x_side, z_side));
                let n = vec3(n.x, 0.0, n.y);
                vert_data.push((v,n).into());
                v_count += 1;
            }
            println!("(1) v_count: {}", v_count);

            // bottom cap slice
            for r_step in (0..(strips+1)).rev() {
                let r = r_stepsz*(r_step as f32);
                let v = vec3(polar_x(r, theta), -R, polar_y(r,theta));
                let n = vec3(0.0, -1.0, 0.0);
                vert_data.push((v,n).into());
                v_count += 1;
            }
            println!("(2) v_count: {}", v_count);
        }

        let nvert = vert_data.len() as u32;
        println!("nvert: {}", nvert);

        // generate indices
        let stride = 3*(strips+1); // each slice has `stride` vertices in it
        for slice in 0..slices {
            let istart = slice * stride;
            index_data.push(istart);
            index_data.push(istart + 1);
            index_data.push((istart + stride + 1) % nvert);

            for step in 1..(stride - 1) {
                let i = istart + step;
                index_data.push(i);
                index_data.push(i + 1);
                index_data.push((i + stride) % nvert);

                index_data.push(i + 1);
                index_data.push((i + stride + 1) % nvert);
                index_data.push((i + stride) % nvert);
            }

            let i = istart + stride - 1;
            index_data.push(i);
            index_data.push(i + 1);
            index_data.push((i + stride) % nvert);
        }

        let vbo = rendergl::VBO::from_data(&vert_data);
        let ibo = rendergl::IBO::from_data(&index_data);
        let vao = rendergl::VAO::new(
            &vbo,
            Some(&ibo),
            GlLayout::Triangles
        );

        Cylinder {
            program: Rc::clone(program),
            _vbo: vbo,
            ibo,
            vao,
            transform: num::one()
        }
    }
}

impl Drawable for Cylinder {
    fn tick(&mut self) {
        self.transform = glm::ext::rotate(&self.transform, 0.005, glm::vec3(0.0, 1.0, 0.0));
    }

    fn draw(&self) -> Result<(), DrawError> {
        self.program.bind();
        self.program.set_uniform("model", &self.transform)?;

        self.vao.bind();
        self.ibo.bind();
        self.vao.draw();
        self.ibo.unbind();
        self.vao.unbind();

        Ok(())
    }
}

fn polar_x(r: f32, theta: f32) -> f32 { r * glm::cos::<>(theta) }
fn polar_y(r: f32, theta: f32) -> f32 { r * glm::sin::<>(theta) }

