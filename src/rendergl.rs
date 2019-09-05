//! Low-level data structures for working with OpenGL objects.
//!
//! Exposes safe abstractions on top of OpenGL API calls.

mod buffer;
mod data;
pub mod shader;
pub mod texture;
pub mod types;
pub mod uniform;

pub use self::buffer::*;
pub use self::data::*;
pub use self::shader::{Program, Shader};
pub use self::uniform::UniformSet;
