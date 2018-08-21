use std;
use std::fs;
use std::ffi::{CString, NulError};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O Error")]
    Io(#[cause] io::Error),
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

pub struct ResourceLoader {
    path_root: PathBuf,
}

impl ResourceLoader {
    /// Construct a ResourceLoader from a relative path to assets directory.
    ///
    /// ```
    /// let res = ResourceLoader("assets/");
    /// ```
    pub fn new(assets: &Path) -> Result<ResourceLoader, Error> {
        let exec_path = std::env::current_exe().map_err(|_| Error::CurrentExeNotFound)?;
        let parent_dir = exec_path.parent().ok_or(Error::CurrentExeNotFound)?;
        Ok( ResourceLoader { path_root: parent_dir.join(assets) } )
    }

    /// Load a resource file named `resource_name` under the `ResourceLoader`'s
    /// assets root directory.
    pub fn load_cstring(&self, resource_name: &Path) -> Result<CString, Error> {
        let mut file = fs::File::open(self.path_root.join(resource_name))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        CString::new(buf).map_err(|e| Error::from(io::Error::from(e)))
    }
}

impl std::fmt::Display for ResourceLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ResourceLoader {{ path_root: {} }}", self.path_root.display())
    }
}
