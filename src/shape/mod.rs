mod triangle;

pub use self::triangle::Triangle;

pub trait Drawable {
    fn init(&mut self) { }
    fn draw(&self);
}

