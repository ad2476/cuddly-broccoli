use std::rc::Rc;
use rendergl::types::*;
use rendergl;
use glm::{self,vec3};
use glm::ext::consts;
use num;

use shape::{Drawable, DrawError};

pub struct Sphere {
    program: Rc<rendergl::Program>,
    _vbo: rendergl::VBO,
    ibo: rendergl::IBO,
    vao: rendergl::VAO,
    transform: glm::Mat4,
}

impl Sphere {
    pub fn new(program: &Rc<rendergl::Program>, lat_strips: u32, lon_slices: u32) -> Sphere {
        let mut vert_data: Vec<rendergl::VertexN> = Vec::new();
        let mut index_data: Vec<u32> = Vec::new();
//        vert_data.reserve(p1*p2 + p2);

        const R: f32 = 0.5;
        let pi: f32 = consts::pi();

        let lon_stepsz: f32 = 2.0*pi/(lon_slices as f32);
        let lat_stepsz: f32 = pi/(lat_strips as f32);

        // generate vertices
        for theta_step in 0..lon_slices {
            let theta = -lon_stepsz*(theta_step as f32);
            for phi_step in 0..(lat_strips+1) {
                let phi = lat_stepsz*(phi_step as f32);

                let v = vec3(spherical_x(R, theta, phi),
                             spherical_y(R, theta, phi),
                             spherical_z(R, theta, phi));
                let n = glm::normalize(v);
                vert_data.push((v,n).into());
            }
        }

        let nvert = vert_data.len() as u32;

        // generate indices
        for slice in 0..lon_slices {
            let istart = slice * (lat_strips + 1);
            index_data.push(istart);
            index_data.push(istart + 1);
            index_data.push((istart + lat_strips + 2) % nvert);

            for strip in 1..(lat_strips - 1) {
                let i = istart + strip;
                index_data.push(i);
                index_data.push((i + lat_strips + 2) % nvert);
                index_data.push((i + lat_strips + 1) % nvert);

                index_data.push(i);
                index_data.push(i + 1);
                index_data.push((i + lat_strips + 2) % nvert)
            }

            let i = istart + lat_strips - 1;
            index_data.push(i);
            index_data.push(i + 1);
            index_data.push((i + lat_strips + 1) % nvert);
        }

        let vbo = rendergl::VBO::from_data(&vert_data);
        let ibo = rendergl::IBO::from_data(&index_data);
        let vao = rendergl::VAO::new(
            &vbo,
            Some(&ibo),
            GlLayout::Triangles
        );

        Sphere {
            program: Rc::clone(program),
            _vbo: vbo,
            ibo,
            vao,
            transform: num::one()
        }
    }
}

impl Drawable for Sphere {
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

fn spherical_x(r: f32, theta: f32, phi: f32) -> f32 {
    polar_x(r, theta) * glm::sin::<>(phi) // x = r*cos(theta)*sin(phi)
}
fn spherical_y(r: f32, _theta: f32, phi: f32) -> f32 {
    polar_x(r, phi) // y = r*cos(phi)
}
fn spherical_z(r: f32, theta: f32, phi: f32) -> f32 {
    polar_y(r, theta) * glm::sin::<>(phi) // z = r*sin(theta)*sin(phi)
}
