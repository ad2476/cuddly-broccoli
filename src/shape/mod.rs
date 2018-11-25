use rendergl::uniform;
mod triangle;

pub use self::triangle::Triangle;

#[derive(Debug, Fail)]
pub enum DrawError {
    #[fail(display = "Uniform error")]
    UniformError { #[cause] inner: uniform::Error },
}

impl From<uniform::Error> for DrawError {
    fn from(other: uniform::Error) -> Self {
        DrawError::UniformError { inner: other }
    }
}

/// Trait for any object that should be drawable
/// in the scene.
pub trait Drawable {
    fn init(&mut self) -> Result<(), DrawError> { Ok(()) }
    fn tick(&mut self) { }
    fn draw(&self) -> Result<(), DrawError>;
}
