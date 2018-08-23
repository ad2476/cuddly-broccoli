#[macro_use] extern crate failure;

extern crate sdl2;
extern crate gl;
extern crate glm;

pub mod render_gl;
pub mod resources;
pub mod ui;
pub mod shape;

use failure::err_msg;

const FPS: u64 = 60;

fn run() -> Result<(), failure::Error> {
    let mut view = ui::View::new("App", 900, 700).map_err(err_msg)?;
    let scene = ui::Scene::new("assets/").map_err(err_msg)?;

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

        scene.render();
        view.gl_swap_window();

        std::thread::sleep(std::time::Duration::from_nanos(1_000_000_000u64 / FPS));
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
