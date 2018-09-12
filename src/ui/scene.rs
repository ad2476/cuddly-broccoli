use std::path::Path;
use std::rc::Rc;

use render_gl;
use resources::{self, ResourceLoader};
use shape::{self,Drawable};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to initialise ResourceLoader for {}", name)]
    ResourceLoadError { name: String, #[cause] inner: resources::Error },
    #[fail(display = "Failed to create shader")]
    ShaderError { #[cause] inner: render_gl::shader::Error },
    #[fail(display = "Error during rendering")]
    RenderError { #[cause] inner: shape::DrawError},
}

impl From<render_gl::shader::Error> for Error {
    fn from(other: render_gl::shader::Error) -> Self {
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
    _loader: ResourceLoader,
}

impl Scene {
    pub fn new(assets_dir: &str) -> Result<Scene, Error> {
        let loader = ResourceLoader::new(Path::new(assets_dir))
            .map_err(|e| Error::ResourceLoadError { name: assets_dir.into(), inner: e })?;
        println!("{}", loader);

        let shader_program = Rc::new(
            render_gl::Program::from_res(&loader, "shaders/triangle")?
        );

        let triangle1 = shape::Triangle::new(&shader_program);
        let mut shapes: Vec<Box<Drawable>> = Vec::new();
        shapes.push(Box::new(triangle1));

        for shape in &mut shapes {
            shape.init()?;
        }
        Ok(Scene { shapes, _loader: loader })
    }

    pub fn tick(&mut self) {
        for shape in &mut self.shapes {
            shape.tick();
        }
    }

    /// Render the objects in the scene.
    pub fn render(&self) -> Result<(), Error> {
        for shape in &self.shapes {
            shape.draw()?;
        }
        render_gl::Program::bind_default();
        Ok(())
    }
}
