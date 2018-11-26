use glm;

pub struct CameraBuilder {
    eye: glm::Vec3,
    look: glm::Vec3,
    up: glm::Vec3,
    near: f32,
    far: f32,
    ratio: f32,
    fov: f32,
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
            near: 1.0,
            far: 80.0,
            ratio: 1.0,
            fov: glm::ext::pi::<f32,f32>()/3.0
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

pub struct Camera {
    pub perspective: glm::Mat4,
    pub view: glm::Mat4,
    params: CameraBuilder,
}

impl Camera {
    pub fn new(params: CameraBuilder) -> Camera {
        let up = glm::normalize(params.up);
        let view = glm::ext::look_at(params.eye, params.look, up);
        let perspective = Camera::create_perspective(&params);

        Camera { perspective, view, params }
    }

    fn create_perspective(params: &CameraBuilder) -> glm::Mat4 {
        glm::ext::perspective(params.fov, params.ratio, params.near,params.far)
    }

    pub fn set_aspect(&mut self, ratio: f32) {
        self.params.ratio = ratio;
        self.perspective = Camera::create_perspective(&self.params);
    }
}