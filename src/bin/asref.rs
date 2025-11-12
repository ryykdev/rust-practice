use std::convert::AsRef;

fn main() {
    let data = ByteData {
        inner_data: "asref example".to_string(),
    };
    process_as_string(data.as_ref());
}

pub struct ByteData {
    inner_data: String,
}

impl AsRef<str> for ByteData {
    fn as_ref(&self) -> &str {
        &self.inner_data
    }
}

// This function accepts ANY type (T) that implements the AsRef<str> trait.
fn process_as_string(data: &str) {
    // We call the required method: as_ref()
    let s: &str = data.as_ref();
    println!("Processed string: '{}' (Length: {})", s, s.len());
}
