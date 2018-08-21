//! A crate to provide bindings to OpenGL with specific extensions enabled,
//! and with a struct generator instead of the global generator provided by the
//! `gl` crate on crates.io
//!
//! Additionally, provides a feature "debug" which can be enabled as "gl/debug".
//! The main difference with a struct, is OpenGL functions become member functions
//! of a `Gl` struct. Enumerations and types are still static as with the global generator.
//!
//! # Examples
//!
//! ```
//! // In this example, `window` is a glfw:Window
//! let gl = gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);
//! ```

// place generated bindings inside private module
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
pub use bindings::Gl as InnerGl;
use std::rc::Rc;
use std::ops::Deref;

/// Wraps a Gl struct in a Rc to provide reference-counts
/// on heap-allocated memory.
#[derive(Clone)]
pub struct Gl {
    obj: Rc<bindings::Gl>,
}

impl Gl {
    /// Forwards the Gl::load_with constructor
    pub fn load_with<F>(loadfn: F) -> Gl
        where F: FnMut(&'static str) -> *const types::GLvoid
    {
        Gl {
            obj: Rc::new(bindings::Gl::load_with(loadfn)),
        }
    }
}

impl Deref for Gl {
    type Target = bindings::Gl;

    fn deref(&self) -> &bindings::Gl {
        &self.obj
    }
}
