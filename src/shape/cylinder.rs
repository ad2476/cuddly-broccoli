use std::rc::Rc;
use rendergl;
use glm;
use num;

use shape::{Drawable, DrawError, ShapeGL};

/// A `Drawable` cylinder.
pub struct Cylinder {
    program: Rc<rendergl::Program>,
    shapegl: ShapeGL,
    transform: glm::Mat4,
}

impl Cylinder {
    pub fn new(program: &Rc<rendergl::Program>, strips: u32, slices: u32) -> Cylinder {
        let shapegl = ShapeGL::cylinder(strips, slices);
        Cylinder {
            program: Rc::clone(program),
            shapegl,
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

        self.shapegl.draw_vertices();

        Ok(())
    }
}

