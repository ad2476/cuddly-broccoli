use std::path::Path;
use std::rc::Rc;
use gl;
use sdl2::keyboard::Keycode;
use glm::vec3;

use rendergl;
use resources::{self, ResourceLoader};
use shape::{self,Drawable};

use camera::*;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to initialise ResourceLoader for {}", name)]
    ResourceLoadError { name: String, #[cause] inner: resources::Error },
    #[fail(display = "Failed to create shader")]
    ShaderError { #[cause] inner: rendergl::shader::Error },
    #[fail(display = "Error during rendering")]
    RenderError { #[cause] inner: shape::DrawError},
}

impl From<rendergl::shader::Error> for Error {
    fn from(other: rendergl::shader::Error) -> Self {
        Error::ShaderError { inner: other }
    }
}

impl From<shape::DrawError> for Error {
    fn from(other: shape::DrawError) -> Self {
        Error::RenderError { inner: other }
    }
}

/// Scene implementation.
///
/// A scene contains a list of `Drawable` objects to render.
/// The scene will eventually also need to contain things such as
/// cameras, lights, and more.
pub struct Scene {
    shapes: Vec<Box<Drawable>>,
    camera: Camera,
    program: Rc<rendergl::Program>,
    _loader: ResourceLoader,
}

impl Scene {
    pub fn new(assets_dir: &str) -> Result<Scene, Error> {
        let loader = ResourceLoader::new(Path::new(assets_dir))
            .map_err(|e| Error::ResourceLoadError { name: assets_dir.into(), inner: e })?;
        println!("{}", loader);

        let lighting_program = Rc::new(
            rendergl::Program::from_res(&loader, "shaders/shader")?
        );
        let animation_program = Rc::new(
            rendergl::Program::from_res(&loader, "shaders/triangle")?
        );

        let camera = CameraBuilder::new()
            .eye(&vec3(1.5,1.0,1.5))
            .look(&vec3(-1.0, -1.0, -1.0))
            .up(&vec3(1.0, 1.0, 1.0))
            .build();

        let triangle = shape::Triangle::new(&animation_program);
        let sphere = shape::Sphere::new(&lighting_program, 10, 16);

        let mut shapes: Vec<Box<Drawable>> = Vec::new();
        shapes.push(Box::new(sphere));
        shapes.push(Box::new(triangle));
        for shape in &mut shapes {
            shape.init()?;
        }

        Ok(Scene { shapes,  camera, program: lighting_program, _loader: loader })
    }

    pub fn tick(&mut self) {
        for shape in &mut self.shapes {
            shape.tick();
        }
    }

    /// Render the objects in the scene.
    pub fn render(&self) -> Result<(), Error> {
        self.program.bind();
        self.program.set_uniform("view", &self.camera.view)
            .map_err(|e| shape::DrawError::from(e))?;
        self.program.set_uniform("perspective", &self.camera.perspective)
            .map_err(|e| shape::DrawError::from(e))?;

        for shape in &self.shapes {
            shape.draw()?;
        }
        rendergl::Program::bind_default();
        Ok(())
    }

    pub fn on_resize(&mut self, x: i32, y: i32) -> Result<(), Error> {
        self.camera.set_aspect((x as f32)/(y as f32));
        Ok(())
    }

    pub fn on_keydown(&mut self, keycode: &Keycode) -> Result<(), Error> {
        match keycode {
            Keycode::Left => {},
            Keycode::Right => {},
            Keycode::Up => {},
            Keycode::Down => {},
            Keycode::F => {
                unsafe {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                }
            }
            Keycode::L => {
                unsafe {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                }
            }
            _ => {},
        }
        Ok(())
    }
}
