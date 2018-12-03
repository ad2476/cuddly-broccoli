
pub fn linear_index(row: usize, col: usize, num_cols: usize) -> usize {
    row*num_cols + col
}

pub fn polar_x(r: f32, theta: f32) -> f32 { r * glm::cos::<>(theta) }
pub fn polar_y(r: f32, theta: f32) -> f32 { r * glm::sin::<>(theta) }

pub fn spherical_x(r: f32, theta: f32, phi: f32) -> f32 {
    polar_x(r, theta) * glm::sin::<>(phi) // x = r*cos(theta)*sin(phi)
}
pub fn spherical_y(r: f32, _theta: f32, phi: f32) -> f32 {
    polar_x(r, phi) // y = r*cos(phi)
}
pub fn spherical_z(r: f32, theta: f32, phi: f32) -> f32 {
    polar_y(r, theta) * glm::sin::<>(phi) // z = r*sin(theta)*sin(phi)
}