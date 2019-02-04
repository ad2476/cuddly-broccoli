//! Utility functions and data types.
use glm;
use std::f32;

/// A point on a 3D shape primitive.
pub enum SurfacePoint {
    /// Spherical point `r, theta, phi`.
    Sphere { r: f32, theta: f32, phi: f32 },
    /// [Cylindrical point](http://mathworld.wolfram.com/CylindricalCoordinates.html) `r, theta, y`.
    Cylinder { r: f32, theta: f32, y: f32 },
    /// A point on a 2D disk coplanar with `y`. (e.g. a cylinder cap)
    Disk { r: f32, theta: f32, y: f32 },
}

impl SurfacePoint {
    pub const R: f32 = 0.5;

    /// Returns 3D position as cartesian coordinate vector.
    pub fn position(&self) -> glm::Vec3 {
        match *self {
            SurfacePoint::Sphere { r, theta, phi } => glm::vec3(
                spherical_x(r, theta, phi),
                spherical_y(r, theta, phi),
                spherical_z(r, theta, phi),
            ),
            SurfacePoint::Cylinder { r, theta, y } | SurfacePoint::Disk { r, theta, y } => {
                glm::vec3(polar_x(r, theta), y, polar_y(r, theta))
            }
        }
    }

    /// Returns 3D normal for a given point on the shape.
    pub fn normal(&self) -> glm::Vec3 {
        match *self {
            SurfacePoint::Sphere { .. } => glm::normalize(self.position()),
            SurfacePoint::Cylinder { r, theta, .. } => {
                let x_side = polar_x(r, theta);
                let z_side = polar_y(r, theta);
                let n = glm::normalize(glm::vec2(x_side, z_side));
                glm::vec3(n.x, 0.0, n.y)
            }
            SurfacePoint::Disk { y, .. } => glm::vec3(0.0, y.signum(), 0.0),
        }
    }

    /// Returns texture-mapping (uv) coordinates for a given point on the shape.
    pub fn texcoord(&self) -> glm::Vec2 {
        match *self {
            SurfacePoint::Sphere { theta, phi, .. } => {
                let u = -theta / (2.0 * f32::consts::PI);
                let v = phi / f32::consts::PI;
                glm::vec2(u, v)
            }
            SurfacePoint::Cylinder { theta, y, .. } => {
                let u = -theta / (2.0 * f32::consts::PI);
                let v = -y - 0.5;
                glm::vec2(u, v)
            }
            SurfacePoint::Disk { r, theta, y } => {
                let u = polar_x(r, theta) + 0.5;
                let v = 1.0 + y.signum() * (polar_y(r, theta) - 0.5);
                glm::vec2(u, v)
            }
        }
    }
}

pub fn linear_index(row: usize, col: usize, num_cols: usize) -> usize {
    row * num_cols + col
}

pub fn polar_x(r: f32, theta: f32) -> f32 {
    r * glm::cos(theta)
}
pub fn polar_y(r: f32, theta: f32) -> f32 {
    r * glm::sin(theta)
}

pub fn spherical_x(r: f32, theta: f32, phi: f32) -> f32 {
    polar_x(r, theta) * glm::sin(phi) // x = r*cos(theta)*sin(phi)
}
pub fn spherical_y(r: f32, _theta: f32, phi: f32) -> f32 {
    polar_x(r, phi) // y = r*cos(phi)
}
pub fn spherical_z(r: f32, theta: f32, phi: f32) -> f32 {
    polar_y(r, theta) * glm::sin(phi) // z = r*sin(theta)*sin(phi)
}
