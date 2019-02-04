use image::DynamicImage;
use std::path::Path;

use camera::Camera;
use rendergl;
use resources;
use shape::*;

/// Shape that uses the "shader" vertex and fragment shaders.
pub struct ShaderShape {
    program: rendergl::Program,
    shapegl: ShapeGL,
    texture: rendergl::texture::Texture,
    transform: glm::Mat4,
    time: f32,
}
type ShaderVertex = rendergl::VertexNT;
impl ShaderShape {
    fn load_texture(
        loader: &resources::ResourceLoader,
        path: &Path,
    ) -> Result<rendergl::texture::Texture, resources::Error> {
        let img = loader
            .load_image(path)
            .map(|i| DynamicImage::ImageRgba8(i.to_rgba()))?;
        let tex = rendergl::texture::Texture::from_image(&img);
        rendergl::texture::TextureParameters::new()
            .wrap_method2d(rendergl::texture::WrapMethod::Repeat)
            .filter_method(rendergl::texture::FilterMethod::Linear)
            .apply_to(&tex);
        Ok(tex)
    }

    pub fn new(
        loader: &resources::ResourceLoader,
        shapegl: ShapeGL,
        texture_path: &Path,
    ) -> Result<ShaderShape, InitError> {
        let program = rendergl::Program::from_res(loader, "shaders/shader")?;
        let texture = ShaderShape::load_texture(loader, texture_path)?;
        Ok(ShaderShape {
            program,
            shapegl,
            texture,
            transform: num::one(),
            time: 0.0,
        })
    }

    pub fn sphere(
        loader: &resources::ResourceLoader,
        lat_strips: u32,
        lon_slices: u32,
    ) -> Result<ShaderShape, InitError> {
        let tex_path = Path::new("images/chessboard.png");
        let shapegl = ShapeGL::sphere::<ShaderVertex>(lat_strips, lon_slices);
        ShaderShape::new(loader, shapegl, &tex_path)
    }

    pub fn cylinder(
        loader: &resources::ResourceLoader,
        strips: u32,
        slices: u32,
    ) -> Result<ShaderShape, InitError> {
        let tex_path = Path::new("images/chessboard.png");
        let shapegl = ShapeGL::cylinder::<ShaderVertex>(strips, slices);
        ShaderShape::new(loader, shapegl, &tex_path)
    }
}

impl Drawable for ShaderShape {
    fn tick(&mut self) {
        self.time += 1.0;
        self.transform = glm::ext::rotate(&self.transform, 0.005, glm::vec3(0.0, 1.0, 0.0));
    }

    fn draw(&self, camera: &Camera) -> Result<(), DrawError> {
        self.program.bind();
        self.texture.bind();
        self.program.set_uniform("view", &camera.view)?;
        self.program
            .set_uniform("perspective", &camera.perspective)?;
        self.program.set_uniform("model", &self.transform)?;
        self.program.set_uniform("u_time", &self.time)?;

        self.shapegl.draw_vertices();
        self.texture.unbind();

        Ok(())
    }
}
