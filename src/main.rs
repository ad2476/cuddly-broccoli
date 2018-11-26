#[macro_use] extern crate failure;

extern crate sdl2;
extern crate gl;
extern crate glm;
extern crate num;

pub mod rendergl;
pub mod resources;
pub mod ui;
pub mod shape;
pub mod camera;

use failure::err_msg;
use sdl2::event::{WindowEvent, Event};
use sdl2::keyboard::Keycode;

const FPS: u64 = 60;

const SCREEN_WIDTH: u32 = 900;
const SCREEN_HEIGHT: u32 = 700;

fn run() -> Result<(), failure::Error> {
    let mut view = ui::View::new("App", SCREEN_WIDTH, SCREEN_HEIGHT).map_err(err_msg)?;
    let mut scene = ui::Scene::new("assets/")?;
    scene.on_resize(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)?;

    'main: loop {
        for event in view.poll_events() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                Event::Window { win_event: WindowEvent::Resized(x,y), .. } => {
                    scene.on_resize(x, y)?;
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    scene.on_keydown(&key)?;
                }
                _ => {},
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        scene.tick();
        scene.render()?;
        view.gl_swap_window();

        std::thread::sleep(std::time::Duration::from_nanos(1_000_000_000u64 / FPS));
    }

    Ok(())
}

// Generate backtrace for failure types
// http://nercury.github.io/rust/opengl/tutorial/2018/02/15/opengl-in-rust-from-scratch-08-failure.html
fn failure_backtrace(e: failure::Error) -> String {
    use std::fmt::Write;

    let mut result = String::new();
    for (i, cause) in e.iter_chain().collect::<Vec<_>>().into_iter().rev().enumerate() {
        if i > 0 {
            let _ = writeln!(&mut result, "which caused the following error:");
        }
        let _ = write!(&mut result, " {}", cause);
        if let Some(backtrace) = cause.backtrace() {
            let bt_str = format!("{}", backtrace);
            if bt_str.len() > 0 {
                let _ = writeln!(&mut result, " at {}", backtrace);
            } else {
                let _ = writeln!(&mut result);
            }
        }
    }

    result
}

fn main() {
    if let Err(e) = run() {
        println!("{}", failure_backtrace(e));
    }
}
