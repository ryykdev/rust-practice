use std::{fmt::Display, io};

fn main() {
    // enum errors and opaque errors
    let result = server();
    match result {
        Ok(_) => println!("server streamed successfully"),
        Err(e) => println!("error {e}"),
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

fn server() -> Result<(), CopyError> {
    let disk_data = load_disk_data()?;
    stream_to_user(disk_data)
}

fn load_disk_data() -> Result<String, CopyError> {
    let s = String::from("some disk data");
    //Err(CopyError::In(std::io::Error::new(
    //    io::ErrorKind::NotFound,
    //    "no disk",
    //)))
    Ok(s)
}

fn stream_to_user(s: String) -> Result<(), CopyError> {
    println!("streaming to user...{s}");
    Ok(())
}
