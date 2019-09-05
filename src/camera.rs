//! Camera implementation(s).
//!
//! The current camera implementation is a perspective camera. It supports a few basic
//! operations. To render with a `Camera`, make sure to pass its perspective and view matrices to
//! a shader which understands how to apply the homographic transform.

use glm;
use num;

/// Builder pattern for constructing a `Camera` out of parameters.
pub struct CameraBuilder {
    pub eye: glm::Vec3,
    pub look: glm::Vec3,
    pub up: glm::Vec3,
    pub near: f32,
    pub far: f32,
    pub ratio: f32,
    pub fov: f32,
}

impl CameraBuilder {
    /// Construct the default camera parameters.
    ///
    /// By default, specifies a camera positioned at `(0, 0, 2)`,
    /// looking at the origin, oriented such that up is along the positive y-axis.
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            eye: glm::vec3(0.0, 0.0, 2.0),
            look: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            near: 0.5,
            far: 50.0,
            ratio: 1.0,
            fov: glm::ext::pi::<f32, f32>() / 3.0,
        }
    }

    pub fn eye(mut self, eye: &glm::Vec3) -> CameraBuilder {
        self.eye = eye.clone();
        self
    }

    pub fn look(mut self, look: &glm::Vec3) -> CameraBuilder {
        self.look = look.clone();
        self
    }

    pub fn up(mut self, up: &glm::Vec3) -> CameraBuilder {
        self.up = up.clone();
        self
    }

    pub fn near_clip(mut self, z: f32) -> CameraBuilder {
        self.near = z;
        self
    }

    pub fn far_clip(mut self, z: f32) -> CameraBuilder {
        self.far = z;
        self
    }

    /// Ratio of width to height.
    pub fn aspect_ratio(mut self, ratio: f32) -> CameraBuilder {
        self.ratio = ratio;
        self
    }

    /// Set field-of-view in terms of radians over the y-axis.
    pub fn fov(mut self, fov: f32) -> CameraBuilder {
        self.fov = fov;
        self
    }

    pub fn build(self) -> Camera {
        Camera::new(self)
    }
}

/// Perspective camera implementation.
pub struct Camera {
    pub perspective: glm::Mat4,
    pub view: glm::Mat4,
    params: CameraBuilder,
}

impl Camera {
    /// Construct a new `Camera` by consuming a `CameraBuilder` instance.
    pub fn new(params: CameraBuilder) -> Camera {
        let view = Camera::create_view(&params);
        let perspective = Camera::create_perspective(&params);

        Camera {
            perspective,
            view,
            params,
        }
    }

    pub fn params(&self) -> &CameraBuilder {
        &self.params
    }

    fn create_perspective(params: &CameraBuilder) -> glm::Mat4 {
        glm::ext::perspective(params.fov, params.ratio, params.near, params.far)
    }

    fn create_view(params: &CameraBuilder) -> glm::Mat4 {
        glm::ext::look_at(params.eye, params.look, glm::normalize(params.up))
    }

    /// Set a new aspect ratio. This will rebuild the camera's perspective transform.
    pub fn set_aspect(&mut self, ratio: f32) {
        self.params.ratio = ratio;
        self.perspective = Camera::create_perspective(&self.params);
    }

    /// Translate the camera along the given vector in world space. This will rebuild the camera's
    /// view transform.
    pub fn translate(&mut self, v: glm::Vec3) {
        self.params.eye = self.params.eye + v;
        self.view = Camera::create_view(&self.params);
    }

    /// Zoom by translating along the camera's `look` vector. This will rebuild the camera's view
    /// transform.
    pub fn zoom(&mut self, delta: f32) {
        let v = self.params.look * delta;
        self.translate(v);
    }

    /// Rotate the camera around a fixed axis (in world-space), while looking at the origin.
    ///
    /// * `angle`: in radians
    /// * `axis`: in world-space
    pub fn orbit(&mut self, angle: f32, axis: &glm::Vec3) {
        let rotate = glm::ext::rotate(&num::one(), angle, *axis);
        let eye = {
            let eye = &self.params.eye;
            (rotate * glm::vec4(eye.x, eye.y, eye.z, 1.0)).truncate(3)
        };
        let up = {
            let up = &self.params.up;
            (rotate * glm::vec4(up.x, up.y, up.z, 0.0)).truncate(3)
        };
        self.params.eye = eye;
        self.params.up = up;
        self.params.look = -eye;
        self.view = Camera::create_view(&self.params);
    }
}
