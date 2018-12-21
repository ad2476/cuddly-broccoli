//! 3D mesh implementation.

use std::rc::Rc;
use std::cmp::{min,max};
use glm;
use num;
use util;
use rendergl::{Program, VertexN, types};
use shape::{ShapeGL, Drawable, DrawError};

// TODO: will need to refactor into a generic shape object thingy
/// Implements `Drawable` to render a 3D mesh.
pub struct MeshObject {
    program: Rc<Program>,
    shapegl: ShapeGL,
    transform: glm::Mat4,
    time: u32,
}

impl Drawable for MeshObject {
    fn tick(&mut self) {
        self.time += 1;
    }

    fn draw(&self) -> Result<(), DrawError> {
        self.program.bind();
        self.program.set_uniform("model", &self.transform)?;
        self.program.set_uniform("u_time", &self.time)?;
        self.shapegl.draw_vertices();
        Ok(())
    }
}

/// 3D point data for a mesh. Consumes itself to construct a `MeshObject`.
pub struct DepthMesh {
    mesh_data: Vec<glm::Vec3>,
    size: glm::Vec3,
    num_rows: usize,
    num_cols: usize,
}

impl DepthMesh {

    /// Create a new `DepthMesh` given a 2D grid of depth samples.
    pub fn new(depth_map: &[f32], num_rows: usize, num_cols: usize) -> DepthMesh {
        let mut mesh_data: Vec<glm::Vec3> = Vec::with_capacity(num_rows*num_cols);

        // construct a mesh of unit scale
        for i in 0..num_rows {
            for j in 0..num_cols {
                let x = (i as f32) / (num_rows as f32) - 0.5;
                let y = -depth_map[util::linear_index(i, j, num_cols)];
                let z = (j as f32) / (num_cols as f32) - 0.5;
                mesh_data.push(glm::vec3(x, y, z));
            }
        }

        DepthMesh {
            mesh_data,
            size: glm::vec3(2.0, 1.0, 2.0),
            num_rows,
            num_cols
        }
    }

    /// Constructs vertex data out of this `DepthMesh`'s 3D point cloud and returns a `MeshObject`
    /// for rendering with OpenGL.
    pub fn build_shape(&self, program: &Rc<Program>) -> MeshObject {
        let shapegl = self.init_buffers();
        MeshObject {
            program: Rc::clone(program),
            shapegl,
            transform: glm::ext::scale(&num::one(), self.size),
            time: 0
        }
    }

    fn push_indices(&self, index_data: &mut Vec<u32>, p1: (i32, i32), p2: (i32, i32)) {
        let ix1 = util::linear_index(p1.0 as usize, p1.1 as usize, self.num_cols) as u32;
        let ix2 = util::linear_index(p2.0 as usize, p2.1 as usize, self.num_cols) as u32;
        index_data.push(ix1);
        index_data.push(ix2);
    }

    fn init_buffers(&self) -> ShapeGL {
        let mut vertex_data: Vec<VertexN> = Vec::with_capacity(self.mesh_data.len());
        let mut index_data: Vec<u32> = Vec::new();

        let num_rows = self.num_rows as i32;
        let num_cols = self.num_cols as i32;
        for i in 0..num_rows {
            for j in 0..num_cols {
                let v = self.get_position(i, j);
                let n = self.get_normal(i, j);
                vertex_data.push((*v,n).into());
            }
        }

        for i in 0..(num_rows-1) {
            for j in (0..num_cols).rev() {
                self.push_indices(&mut index_data, (i,j), (i+1,j));
            }
            self.push_indices(&mut index_data, (i+1,0), (i+1,num_cols-1));
        }

        ShapeGL::new(&vertex_data, &index_data, types::GlLayout::TriangleStrip)
    }

    fn get_position(&self, row: i32, col: i32) -> &glm::Vec3 {
        let i = min(max(0, row) as usize, self.num_rows - 1);
        let j = min(max(0, col) as usize, self.num_cols - 1);
        &self.mesh_data[util::linear_index(i, j, self.num_cols)]
    }

    fn get_normal(&self, row: i32, col: i32) -> glm::Vec3 {
        let p = self.get_position(row, col);
        let mut normals: Vec<glm::Vec3> = Vec::new();
        let inc = glm::ext::quarter_pi::<f32,f32>();

        // compute neighbouring normals
        for i in 0..8 {
            let theta = (i as f32) * inc;
            let r0 = glm::sin(theta).round() as i32;
            let c0 = glm::cos(theta).round() as i32;
            let r1 = glm::sin(theta + inc).round() as i32;
            let c1 = glm::cos(theta + inc).round() as i32;

            let n0 = *self.get_position(row + r0, col + c0);
            let n1 = *self.get_position(row + r1, col + c1);
            normals.push(glm::cross(n0 - *p, n1 - *p));
        }

        // normal is average of neighbouring normals
        let mut n = glm::to_vec3(0.0);
        for v in &normals {
            n = n + *v;
        }
        n = n / (normals.len() as f32);
        glm::normalize(n)
    }
}