use std::{error::Error, fmt::Display, io};

fn main() {
    // downcasting an error back to its enum variant
    // this is only possible if you have access to the
    // enum Error
    let result = server();

    match result {
        Ok(t) => println!("server Ok"),
        Err(e) => {
            println!("downcasting error");
            let error = e.downcast_ref::<CopyError>().unwrap();
            match error {
                CopyError::In(e) => println!("In({e})"),
                CopyError::Out(e) => println!("Out({e})"),
            }
        }
    }
}

#[derive(Debug)]
pub enum CopyError {
    In(std::io::Error),
    Out(std::io::Error),
}

// Display trait is required for standard Error
impl Display for CopyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CopyError::In(e) => write!(f, "Input error: {}", e),
            CopyError::Out(e) => write!(f, "Output error: {}", e),
        }
    }
}
impl Error for CopyError {}

fn server() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    server_internals()?;
    Ok(())
}

fn server_internals() -> Result<(), CopyError> {
    Err(CopyError::In(std::io::Error::new(
        io::ErrorKind::NotFound,
        "no disk",
    )))
}
