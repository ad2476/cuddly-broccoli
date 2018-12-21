use sdl2;
use gl;
use std;

/// Owns handles to SDL and GL contexts. Processes events and renders OpenGL scene.
pub struct View {
    _sdl_ctx: sdl2::Sdl,
    _gl_ctx: sdl2::video::GLContext,
    window_ctx: sdl2::video::Window,
    event_pump: sdl2::EventPump,
}

impl View {

    /// Create a new `View`. Initialises SDL, window context, GL context.
    ///
    /// Sets up the OpenGL viewport, background colour. Enables depth testing, back-face culling,
    /// and specifies counter-clockwise triangle winding order.
    pub fn new(window_title: &str, width: u32, height: u32) -> Result<View, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);

        let window_ctx = video_subsystem
                        .window(window_title, width, height)
                        .opengl()
                        .resizable()
                        .build()
                        .map_err(|e| format!("{}", e))?;

        let gl_ctx = window_ctx.gl_create_context()?;
        let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::ClearColor(0.6, 0.6, 0.6, 1.0);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::FrontFace(gl::CCW);
        }

        let event_pump = sdl.event_pump()?;
        Ok(View {
            _sdl_ctx: sdl,
            _gl_ctx: gl_ctx,
            event_pump,
            window_ctx })
    }

    pub fn poll_events(&mut self) -> sdl2::event::EventPollIterator {
        self.event_pump.poll_iter()
    }

    pub fn gl_swap_window(&mut self) {
        self.window_ctx.gl_swap_window();
    }
}
