use rendergl::uniform;
mod triangle;
mod sphere;
mod cylinder;

pub use self::triangle::Triangle;
pub use self::sphere::Sphere;
pub use self::cylinder::Cylinder;

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
