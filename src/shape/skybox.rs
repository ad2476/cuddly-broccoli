use std::path::Path;
use image::DynamicImage;
use glm;

use camera::Camera;
use resources;
use rendergl;
use shape::{Drawable, DrawError, ShapeGL, InitError};

/// A `Drawable` skybox.
pub struct Skybox {
    program: rendergl::Program,
    shapegl: ShapeGL,
    texture: rendergl::texture::Texture,
}

impl Skybox {
    fn cube_shape() -> ShapeGL {
        let vertex_data: Vec<rendergl::VertexP> = vec![
            glm::vec3(-1.0,  1.0, -1.0).into(),
            glm::vec3(-1.0, -1.0, -1.0).into(),
            glm::vec3(1.0, -1.0, -1.0).into(),
            glm::vec3(1.0, -1.0, -1.0).into(),
            glm::vec3(1.0,  1.0, -1.0).into(),
            glm::vec3(-1.0,  1.0, -1.0).into(),
            glm::vec3(-1.0, -1.0,  1.0).into(),
            glm::vec3(-1.0, -1.0, -1.0).into(),
            glm::vec3(-1.0,  1.0, -1.0).into(),
            glm::vec3(-1.0,  1.0, -1.0).into(),
            glm::vec3(-1.0,  1.0,  1.0).into(),
            glm::vec3(-1.0, -1.0,  1.0).into(),
            glm::vec3(1.0, -1.0, -1.0).into(),
            glm::vec3(1.0, -1.0,  1.0).into(),
            glm::vec3(1.0,  1.0,  1.0).into(),
            glm::vec3(1.0,  1.0,  1.0).into(),
            glm::vec3(1.0,  1.0, -1.0).into(),
            glm::vec3(1.0, -1.0, -1.0).into(),
            glm::vec3(-1.0, -1.0,  1.0).into(),
            glm::vec3(-1.0,  1.0,  1.0).into(),
            glm::vec3(1.0,  1.0,  1.0).into(),
            glm::vec3(1.0,  1.0,  1.0).into(),
            glm::vec3(1.0, -1.0,  1.0).into(),
            glm::vec3(-1.0, -1.0,  1.0).into(),
            glm::vec3(-1.0,  1.0, -1.0).into(),
            glm::vec3(1.0,  1.0, -1.0).into(),
            glm::vec3(1.0,  1.0,  1.0).into(),
            glm::vec3(1.0,  1.0,  1.0).into(),
            glm::vec3(-1.0,  1.0,  1.0).into(),
            glm::vec3(-1.0,  1.0, -1.0).into(),
            glm::vec3(-1.0, -1.0, -1.0).into(),
            glm::vec3(-1.0, -1.0,  1.0).into(),
            glm::vec3(1.0, -1.0, -1.0).into(),
            glm::vec3(1.0, -1.0, -1.0).into(),
            glm::vec3(-1.0, -1.0,  1.0).into(),
            glm::vec3(1.0, -1.0,  1.0).into()
        ];
        let n = vertex_data.len() as u32;
        let index_data: Vec<u32> = (0..n).collect();

        ShapeGL::new(&vertex_data, &index_data, rendergl::types::GlLayout::Triangles)
    }

    fn load_texture(loader: &resources::ResourceLoader) -> Result<rendergl::texture::Texture, resources::Error> {
        let faces = vec!["right", "left", "top", "bottom", "front", "back"];
        let path_root = Path::new("images/skybox_lowres/");

        // load each image as "images/skybox/{name}.jpg" into a DynamicImage:
        let images = faces.iter()
            .map(|name|
                loader.load_image(
                    path_root.join(name).with_extension("jpg").as_path())
                    .map(|i| DynamicImage::ImageBgra8(i.to_bgra()))
            )
            .collect::<Result<Vec<DynamicImage>, resources::Error>>()?;

        // if it panics here, something's very wrong...
        let texture = rendergl::texture::Texture::cubemap(&images).unwrap();
        rendergl::texture::TextureParameters::new()
            .wrap_method3d(rendergl::texture::WrapMethod::ClampToEdge)
            .filter_method(rendergl::texture::FilterMethod::Linear)
            .apply_to(&texture);
        Ok(texture)
    }

    pub fn new(loader: &resources::ResourceLoader) -> Result<Skybox, InitError> {
        let shapegl = Skybox::cube_shape();
        let program = rendergl::Program::from_res(loader, "shaders/skybox")?;
        let texture = Skybox::load_texture(loader)?;
        Ok(Skybox {
            program,
            shapegl,
            texture,
        })
    }
}

impl Drawable for Skybox {
    fn draw(&self, camera: &Camera) -> Result<(), DrawError> {
        self.program.bind();
        self.texture.bind();
        self.program.set_uniform("view", &camera.view)?;
        self.program.set_uniform("perspective", &camera.perspective)?;

        self.shapegl.draw_vertices();
        self.texture.unbind();

        Ok(())
    }
}
