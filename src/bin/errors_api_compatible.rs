use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    call_api()?;
    Ok(())
}

fn call_api() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    Ok(())
}
