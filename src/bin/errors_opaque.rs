use std::{error::Error, fmt::Display, io};

fn main() -> Result<(), Box<dyn Error>> {
    // the "?" will convert the CopyError to the opaque error
    // automatically using the Debug trait implementation
    server()?;
    Ok(())
}

#[derive(Debug)]
pub enum CopyError {
    In(std::io::Error),
    Out(std::io::Error),
}

// required for standard Error - needs to be printable
impl Display for CopyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CopyError::In(e) => write!(f, "CopyError: {}", e),
            CopyError::Out(e) => write!(f, "CopyError: {}", e),
        }
    }
}

// required for trait object
impl std::error::Error for CopyError {}

fn server() -> Result<(), CopyError> {
    Err(CopyError::In(io::Error::new(
        io::ErrorKind::NotFound,
        "no disk",
    )))
}
