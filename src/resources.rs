//! Resource loading system.
//!
//! Use `ResourceLoader` for loading files from a resource root path.

use std;
use std::ffi::{CString, NulError};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use image;
use tobj;

/// Error types for resource loading.
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O Error")]
    Io(#[cause] io::Error),
    #[fail(display = "Image Decode Error")]
    DecodeError(#[cause] image::ImageError),
    #[fail(display = "OBJ Load Error")]
    ObjLoadError(String),
    #[fail(display = "Failed to get executable path")]
    CurrentExeNotFound,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

impl From<NulError> for Error {
    fn from(other: NulError) -> Self {
        Error::from(io::Error::from(other))
    }
}

impl From<image::ImageError> for Error {
    fn from(other: image::ImageError) -> Self {
        Error::DecodeError(other)
    }
}

impl From<tobj::LoadError> for Error {
    fn from(other: tobj::LoadError) -> Self {
        Error::ObjLoadError(other.to_string())
    }
}

/// Safely loads data from a resource path root.
pub struct ResourceLoader {
    path_root: PathBuf,
}

impl ResourceLoader {
    /// Construct a ResourceLoader from a relative path to assets directory.
    ///
    /// # Example
    ///
    /// ```
    /// let res = ResourceLoader("assets/");
    /// ```
    pub fn new(assets: &Path) -> Result<ResourceLoader, Error> {
        let exec_path = std::env::current_exe().map_err(|_| Error::CurrentExeNotFound)?;
        let parent_dir = exec_path.parent().ok_or(Error::CurrentExeNotFound)?;
        Ok(ResourceLoader {
            path_root: parent_dir.join(assets),
        })
    }

    /// Load a resource file named `resource_name` under the `ResourceLoader`'s
    /// assets root directory.
    pub fn load_cstring(&self, resource_name: &Path) -> Result<CString, Error> {
        let mut file = fs::File::open(self.path_root.join(resource_name))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        CString::new(buf).map_err(|e| Error::from(io::Error::from(e)))
    }

    /// Load an OBJ model. Wraps `tobj::load_obj` to handle relative resource paths.
    pub fn load_obj(
        &self,
        resource_name: &Path,
    ) -> Result<(Vec<tobj::Model>, Vec<tobj::Material>), Error> {
        tobj::load_obj(&self.path_root.join(resource_name)).map_err(|e| e.into())
    }

    /// Load an image `resource_name` under the `ResourceLoader` root assets directory.
    pub fn load_image(&self, resource_name: &Path) -> Result<image::DynamicImage, Error> {
        image::open(self.path_root.join(resource_name)).map_err(|e| e.into())
    }
}

impl std::fmt::Display for ResourceLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "ResourceLoader {{ path_root: {} }}",
            self.path_root.display()
        )
    }
}
