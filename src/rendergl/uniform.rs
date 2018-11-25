use gl;
use glm;

/// Error enum for uniforms
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to find uniform '{}'. ", name)]
    NotFoundError { name: String },
    #[fail(display = "Uniform index out of bounds: {}", index)]
    IndexError { index: usize },
}

/// Trait to separate/abstract logic of setting uniforms in a program.
///
/// Implementing `UniformSet` for a type allows for setting a uniform
/// of corresponding type in a shader program. This usually involves
/// wrapping a call to `gl::Uniform*` methods.
pub trait UniformSet {
    fn set_uniform_gl(&self, location: gl::types::GLint);
}

impl UniformSet for i32 {
    fn set_uniform_gl(&self, loc: gl::types::GLint) {
        unsafe { gl::Uniform1i(loc, *self); }
    }
}

impl UniformSet for f32 {
    fn set_uniform_gl(&self, loc: gl::types::GLint) {
        unsafe { gl::Uniform1f(loc, *self); }
    }
}

impl UniformSet for glm::Vec2 {
    fn set_uniform_gl(&self, loc: gl::types::GLint) {
        unsafe { gl::Uniform2fv(loc, 1, self.as_array().as_ptr()); }
    }
}

impl UniformSet for glm::Vec3 {
    fn set_uniform_gl(&self, loc: gl::types::GLint) {
        unsafe { gl::Uniform3fv(loc, 1, self.as_array().as_ptr()); }
    }
}

impl UniformSet for glm::Vec4 {
    fn set_uniform_gl(&self, loc: gl::types::GLint) {
        unsafe { gl::Uniform4fv(loc, 1, self.as_array().as_ptr()); }
    }
}

impl UniformSet for glm::IVec2 {
    fn set_uniform_gl(&self, loc: gl::types::GLint) {
        unsafe { gl::Uniform2iv(loc, 1, self.as_array().as_ptr()); }
    }
}

impl UniformSet for glm::IVec3 {
    fn set_uniform_gl(&self, loc: gl::types::GLint) {
        unsafe { gl::Uniform3iv(loc, 1, self.as_array().as_ptr()); }
    }
}

impl UniformSet for glm::IVec4 {
    fn set_uniform_gl(&self, loc: gl::types::GLint) {
        unsafe { gl::Uniform4iv(loc, 1, self.as_array().as_ptr()); }
    }
}

impl UniformSet for glm::Mat2 {
    fn set_uniform_gl(&self, loc: gl::types::GLint) {
        unsafe { gl::UniformMatrix2fv(loc, 1, gl::FALSE, self[0].as_array().as_ptr()); }
    }
}

impl UniformSet for glm::Mat3 {
    fn set_uniform_gl(&self, loc: gl::types::GLint) {
        unsafe { gl::UniformMatrix3fv(loc, 1, gl::FALSE, self[0].as_array().as_ptr()); }
    }
}

impl UniformSet for glm::Mat4 {
    fn set_uniform_gl(&self, loc: gl::types::GLint) {
        unsafe { gl::UniformMatrix4fv(loc, 1, gl::FALSE, self[0].as_array().as_ptr()); }
    }
}

