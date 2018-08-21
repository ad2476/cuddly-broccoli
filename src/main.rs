#[macro_use] extern crate failure;

extern crate sdl2;
extern crate gl;
extern crate glm;

pub mod render_gl;
pub mod resources;
pub mod ui;

use std::path::Path;
use resources::ResourceLoader;
use render_gl::types::*;
use render_gl::{VBOAttribMarker};

const FPS: u64 = 60;

fn main() {
    let mut view = ui::View::new("App", 900, 700).unwrap();

    let res_loader = ResourceLoader::new(Path::new("assets/")).unwrap();
    println!("{}", res_loader);
    let shader_program = render_gl::Program::from_res(&res_loader, "shaders/triangle").unwrap();

    let vertex_data: Vec<render_gl::Vertex> = vec![
        (-0.5, -0.5, 0.0).into(),
        (0.5, -0.5, 0.0).into(),
        (0.0, 0.5, 0.0).into()
    ];
    let markers: Vec<VBOAttribMarker> = vec![
        VBOAttribMarker::new(
            ShaderAttrib::POSITION, 
            VertexAttrib::FLOAT,
            3,
            gl::FALSE,
            0)
    ];
    let triangle_vbo = render_gl::VBO::from_data(
        &vertex_data,
        markers
    );
    let triangle_vao = render_gl::VAO::new(
        &triangle_vbo,
        GlLayout::Triangles
    );

    shader_program.set_used();
    'main: loop {
        for event in view.poll_events() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        triangle_vao.bind();
        triangle_vao.draw();
        triangle_vao.unbind();

        view.gl_swap_window();

        std::thread::sleep(std::time::Duration::from_nanos(1_000_000_000u64 / FPS));
    }
}
