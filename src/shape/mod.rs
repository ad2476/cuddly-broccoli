use glm::ext::consts;
use glm::{self, vec3, vec2};
use rendergl::{self,uniform};
use rendergl::types::*;
use util;

mod triangle;
mod sphere;
mod cylinder;

pub use self::triangle::Triangle;
pub use self::sphere::Sphere;
pub use self::cylinder::Cylinder;

#[derive(Debug, Fail)]
pub enum DrawError {
    #[fail(display = "Uniform error")]
    UniformError { #[cause] inner: uniform::Error },
}

impl From<uniform::Error> for DrawError {
    fn from(other: uniform::Error) -> Self {
        DrawError::UniformError { inner: other }
    }
}

/// Trait for any object that should be drawable
/// in the scene.
pub trait Drawable {
    fn init(&mut self) -> Result<(), DrawError> { Ok(()) }
    fn tick(&mut self) { }
    fn draw(&self) -> Result<(), DrawError>;
}

pub struct ShapeGL {
    _vbo: rendergl::VBO,
    ibo: rendergl::IBO,
    vao: rendergl::VAO,
}

impl ShapeGL {
    pub fn new<T: rendergl::Vertex>(
        vertex_data: &[T],
        indices: &[u32],
        layout: GlLayout
    ) -> ShapeGL {
        let vbo = rendergl::VBO::from_data(vertex_data);
        let ibo = rendergl::IBO::from_data(indices);
        let vao = rendergl::VAO::new(
            &vbo,
            Some(&ibo),
            layout
        );
        ShapeGL { _vbo: vbo, ibo, vao }
    }

    pub fn draw_vertices(&self) {
        self.vao.bind();
        self.ibo.bind();
        self.vao.draw();
        self.ibo.unbind();
        self.vao.unbind();
    }
}

impl ShapeGL {
    /// Generate vertices for a unit sphere.
    ///
    /// `lat_strips`: number of subdivisions in latitude (vertical lod)
    /// `lon_strips`: number of subdivisions in longitude (horizontal lod)
    pub fn sphere(lat_strips: u32, lon_slices: u32) -> ShapeGL {
        let mut vert_data: Vec<rendergl::VertexN> = Vec::new();
        let mut index_data: Vec<u32> = Vec::new();

        const R: f32 = 0.5;
        let pi: f32 = consts::pi();

        let lon_stepsz: f32 = 2.0*pi/(lon_slices as f32);
        let lat_stepsz: f32 = pi/(lat_strips as f32);

        // generate vertices
        for theta_step in 0..lon_slices {
            let theta = -lon_stepsz*(theta_step as f32);
            for phi_step in 0..(lat_strips+1) {
                let phi = lat_stepsz*(phi_step as f32);

                let v = vec3(
                    util::spherical_x(R, theta, phi),
                    util::spherical_y(R, theta, phi),
                    util::spherical_z(R, theta, phi));
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

        ShapeGL::new(&vert_data, &index_data, GlLayout::Triangles)
    }
}

impl ShapeGL {
    pub fn cylinder(strips: u32, slices: u32) -> ShapeGL {
        let mut vert_data: Vec<rendergl::VertexN> = Vec::new();
        let mut index_data: Vec<u32> = Vec::new();

        const R: f32 = 0.5;
        let pi: f32 = consts::pi();

        let theta_stepsz: f32 = 2.0*pi/(slices as f32);
        let r_stepsz: f32 = R/(strips as f32);
        let side_stepsz: f32 = 2.0*R/(strips as f32);

        // generate vertices
        for theta_step in 0..slices {
            let theta = -theta_stepsz*(theta_step as f32);

            // top cap slice
            for r_step in 0..(strips+1) {
                let r = r_stepsz*(r_step as f32);
                let v = vec3(util::polar_x(r, theta), R, util::polar_y(r, theta));
                let n = vec3(0.0, 1.0, 0.0);
                vert_data.push((v,n).into());
            }

            // slice side
            let x_side = util::polar_x(R, theta);
            let z_side = util::polar_y(R, theta);
            for y_step in 0..(strips+1) {
                let y = R - side_stepsz*(y_step as f32);
                let v = vec3(x_side, y, z_side);
                let n = glm::normalize(vec2(x_side, z_side));
                let n = vec3(n.x, 0.0, n.y);
                vert_data.push((v,n).into());
            }

            // bottom cap slice
            for r_step in (0..(strips+1)).rev() {
                let r = r_stepsz*(r_step as f32);
                let v = vec3(util::polar_x(r, theta), -R, util::polar_y(r,theta));
                let n = vec3(0.0, -1.0, 0.0);
                vert_data.push((v,n).into());
            }
        }

        let nvert = vert_data.len() as u32;

        // generate indices
        let stride = 3*(strips+1); // each slice has `stride` vertices in it
        for slice in 0..slices {
            let istart = slice * stride;
            index_data.push(istart);
            index_data.push(istart + 1);
            index_data.push((istart + stride + 1) % nvert);

            for step in 1..(stride - 1) {
                let i = istart + step;
                index_data.push(i);
                index_data.push(i + 1);
                index_data.push((i + stride) % nvert);

                index_data.push(i + 1);
                index_data.push((i + stride + 1) % nvert);
                index_data.push((i + stride) % nvert);
            }

            let i = istart + stride - 1;
            index_data.push(i);
            index_data.push(i + 1);
            index_data.push((i + stride) % nvert);
        }

        ShapeGL::new(&vert_data, &index_data, GlLayout::Triangles)
    }
}
