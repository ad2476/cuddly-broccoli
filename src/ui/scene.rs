use std::path::Path;
use std::rc::Rc;

use render_gl;
use resources::ResourceLoader;
use shape::{self,Drawable};

pub struct Scene {
    shapes: Vec<Box<Drawable>>,
    _loader: ResourceLoader,
}

impl Scene {
    pub fn new(assets_dir: &str) -> Scene {
        let loader = ResourceLoader::new(Path::new(assets_dir)).unwrap();
        println!("{}", loader);
        
        let shader_program = Rc::new(
            render_gl::Program::from_res(&loader, "shaders/triangle").unwrap()
        );

        let triangle1 = shape::Triangle::new(&shader_program);
        let mut shapes: Vec<Box<Drawable>> = Vec::new();
        shapes.push(Box::new(triangle1));

        let data: Vec<render_gl::Vertex> = vec![
            (-0.5, 0.5, 0.0).into(),
            (0.0, -0.5, 0.0).into(),
            (0.5, 0.5, 0.0).into()
        ];
        let triangle2 = shape::Triangle::from_data(data, &shader_program);
        shapes.push(Box::new(triangle2));

        Scene { shapes, _loader: loader }
    }

    pub fn render(&self) {
        for shape in &self.shapes {
            shape.draw();
        }
    }
}
