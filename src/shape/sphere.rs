use std::rc::Rc;
use rendergl;
use num;
use glm;

use shape::{Drawable, DrawError, ShapeGL};

pub struct Sphere {
    program: Rc<rendergl::Program>,
    shapegl: ShapeGL,
    transform: glm::Mat4,
    time: u32,
}

impl Sphere {
    pub fn new(program: &Rc<rendergl::Program>, lat_strips: u32, lon_slices: u32) -> Sphere {
        let shapegl = ShapeGL::sphere(lat_strips, lon_slices);
        Sphere {
            program: Rc::clone(program),
            shapegl,
            transform: num::one(),
            time: 0
        }
    }
}

impl Drawable for Sphere {
    fn tick(&mut self) {
        self.time += 1;
        self.transform = glm::ext::rotate(&self.transform, 0.005, glm::vec3(0.0, 1.0, 0.0));
    }

    fn draw(&self) -> Result<(), DrawError> {
        self.program.bind();
        self.program.set_uniform("model", &self.transform)?;
        self.program.set_uniform("u_time", &self.time)?;

        self.shapegl.draw_vertices();

        Ok(())
    }
}

