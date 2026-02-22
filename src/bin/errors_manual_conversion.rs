use std::fmt::Display;

fn main() {
    let custom_error = CopyError::In(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "disk failure",
    ));
    let opaque_error = Box::from(custom_error);

    println!("{}", opaque_error);
    println!("{:?}", opaque_error);
}

#[derive(Debug)] // standard Error needs Debug
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
