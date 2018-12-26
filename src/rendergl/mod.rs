//! Low-level data structures for working with OpenGL objects.
//!
//! Exposes safe abstractions on top of OpenGL API calls.

pub mod shader;
pub mod uniform;
mod data;
mod buffer;
pub mod types;
pub mod texture;

pub use self::shader::{Shader, Program};
pub use self::data::*;
pub use self::buffer::*;
pub use self::uniform::{UniformSet};
