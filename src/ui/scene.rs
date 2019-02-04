use gl;
use glm::vec3;
use sdl2::keyboard::Keycode;
use std::path::Path;

use mesh;
use rendergl;
use resources::{self, ResourceLoader};
use shape::{self, Drawable};

use camera::*;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to initialise ResourceLoader for {}", name)]
    ResourceLoadError {
        name: String,
        #[cause]
        inner: resources::Error,
    },
    #[fail(display = "Failed to create shader")]
    ShaderError {
        #[cause]
        inner: rendergl::shader::Error,
    },
    #[fail(display = "Error during rendering")]
    RenderError {
        #[cause]
        inner: shape::DrawError,
    },
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

impl From<shape::InitError> for Error {
    fn from(other: shape::InitError) -> Self {
        match other {
            shape::InitError::ResourceError(e) => Error::ResourceLoadError {
                name: "ShaderShape".into(),
                inner: e,
            },
            shape::InitError::ShaderError(e) => Error::ShaderError { inner: e },
        }
    }
}

fn make_mesh(loader: &resources::ResourceLoader) -> Result<mesh::MeshObject, shape::InitError> {
    let mut depth_data = Vec::new();
    let w = 20;
    let h = 20;
    for i in 0..h {
        for j in 0..w {
            let x = (i as f32) / (h as f32) - 0.5;
            let y = (j as f32) / (w as f32) - 0.5;
            depth_data.push(1.0 - (x * x + y * y) - 0.5);
        }
    }
    mesh::DepthMesh::new(&depth_data, h, w).build_shape(&loader)
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
    _loader: ResourceLoader,
}

impl Scene {
    const ROT_DELTA: f32 = 0.01;

    pub fn new(assets_dir: &str) -> Result<Scene, Error> {
        let loader =
            ResourceLoader::new(Path::new(assets_dir)).map_err(|e| Error::ResourceLoadError {
                name: assets_dir.into(),
                inner: e,
            })?;
        println!("{}", loader);

        let camera = CameraBuilder::new()
            .eye(&vec3(1.5, 1.0, 1.5))
            .look(&vec3(-1.5, -1.0, -1.5))
            .up(&vec3(-1.0, 1.0, -1.0))
            .build();

        let sphere = shape::ShaderShape::sphere(&loader, 100, 100)?;
        //        let cylinder = shape::ShaderShape::cylinder(&loader, 50, 50)?;
        let skybox = shape::Skybox::new(&loader)?;

        let mut shapes: Vec<Box<Drawable>> = Vec::new();
        shapes.push(Box::new(sphere));
        shapes.push(Box::new(skybox));
        for shape in &mut shapes {
            shape.init()?;
        }

        Ok(Scene {
            shapes,
            camera,
            _loader: loader,
        })
    }

    pub fn tick(&mut self) {
        for shape in &mut self.shapes {
            shape.tick();
        }
    }

    /// Render the objects in the scene.
    pub fn render(&self) -> Result<(), Error> {
        for shape in &self.shapes {
            shape.draw(&self.camera)?;
        }
        rendergl::Program::bind_default();
        Ok(())
    }

    pub fn on_resize(&mut self, x: i32, y: i32) -> Result<(), Error> {
        self.camera.set_aspect((x as f32) / (y as f32));
        unsafe {
            gl::Viewport(0, 0, x, y);
        }
        Ok(())
    }

    pub fn on_keydown(&mut self, keycode: &Keycode) -> Result<(), Error> {
        match keycode {
            Keycode::Left => {
                self.camera
                    .orbit(-Scene::ROT_DELTA, &glm::vec3(0.0, 1.0, 0.0));
            }
            Keycode::Right => {
                self.camera
                    .orbit(Scene::ROT_DELTA, &glm::vec3(0.0, 1.0, 0.0));
            }
            Keycode::Up => {
                let axis = {
                    let params = self.camera.params();
                    glm::cross(params.up, params.look)
                };
                self.camera.orbit(Scene::ROT_DELTA, &axis);
            }
            Keycode::Down => {
                let axis = {
                    let params = self.camera.params();
                    glm::cross(params.up, params.look)
                };
                self.camera.orbit(-Scene::ROT_DELTA, &axis);
            }
            Keycode::F => unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            },
            Keycode::L => unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            },
            _ => {}
        }
        Ok(())
    }

    pub fn on_scroll(&mut self, delta: i32) -> Result<(), Error> {
        let dir = if delta > 0 { 1.0 } else { -1.0 };
        self.camera.zoom(dir * 0.05);
        Ok(())
    }
}
