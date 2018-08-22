mod triangle;

pub use self::triangle::Triangle;

/// Trait for any object that should be drawable
/// in the scene.
pub trait Drawable {
    fn init(&mut self) { }
    fn draw(&self);
}

