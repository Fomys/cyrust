use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FbError {
    IoctlFailed { code: libc::c_int },
}

impl fmt::Display for FbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            FbError::IoctlFailed { code } => write!(f, "Ioctl failed: error code {}", code),
        }
    }
}

impl Error for FbError {}
