use std::path::Path;
use std::rc::Rc;
use gl;
use sdl2::keyboard::Keycode;
use glm::vec3;

use rendergl;
use image::DynamicImage;
use resources::{self, ResourceLoader};
use shape::{self,Drawable};
use mesh;

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

fn make_mesh(program: &Rc<rendergl::Program>) -> mesh::MeshObject {
    let mut depth_data = Vec::new();
    let w = 20;
    let h = 20;
    for i in 0..h {
        for j in 0..w {
            let x = (i as f32) / (h as f32) - 0.5;
            let y = (j as f32) / (w as f32) - 0.5;
            depth_data.push(1.0 - (x*x + y*y) - 0.5);
        }
    }
    mesh::DepthMesh::new(&depth_data, h, w).build_shape(&program)
}

/// Scene implementation.
///
/// A scene contains a list of `Drawable` objects and a program to render them with.
/// Currently, the camera is also part of the scene.
///
/// The scene will eventually also need to contain things such as lights and a scenegraph.
pub struct Scene {
    shapes: Vec<Box<Drawable>>,
    camera: Camera,
    texture: rendergl::texture::Texture,
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
//        let sphere = shape::Sphere::new(&lighting_program, 50, 50);
//        let cylinder = shape::Cylinder::new(&lighting_program, 50, 50);
//        let mesh = make_mesh(&lighting_program);

        let img_path = "images/chessboard.png";
        let img = loader.load_image(Path::new(img_path))
            .map(|i| DynamicImage::ImageRgba8(i.to_rgba()))
            .map_err(|e| Error::ResourceLoadError { name: img_path.into(), inner: e })?;
        let texture = rendergl::texture::Texture::from_image(&img);

        rendergl::texture::TextureParameters::new()
            .wrap_method(rendergl::texture::WrapMethod::ClampToEdge)
            .filter_method(rendergl::texture::FilterMethod::Linear)
            .apply_to(&texture);

        let mut shapes: Vec<Box<Drawable>> = Vec::new();
//        shapes.push(Box::new(sphere));
        shapes.push(Box::new(triangle));
//        shapes.push(Box::new(mesh));
        for shape in &mut shapes {
            shape.init()?;
        }

        Ok(Scene {
            shapes,
            camera,
            texture,
            program: lighting_program,
            _loader: loader
        })
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
        self.texture.bind();

        for shape in &self.shapes {
            shape.draw()?;
        }
        rendergl::Program::bind_default();
        self.texture.unbind();
        Ok(())
    }

    pub fn on_resize(&mut self, x: i32, y: i32) -> Result<(), Error> {
        self.camera.set_aspect((x as f32)/(y as f32));
        unsafe { gl::Viewport(0, 0, x, y); }
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

    pub fn on_scroll(&mut self, delta: i32) -> Result<(), Error> {
        let dir = if delta > 0 { 1.0 } else { -1.0 };
        self.camera.zoom(dir*0.05);
        Ok(())
    }
}
