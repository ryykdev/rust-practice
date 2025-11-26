use std::convert::AsRef;

fn main() {
    let data = ByteData {
        meta_data: 42,
        inner_data: InnerData {
            data: "AsRef inner data".to_string(),
        },
    };
    println!("outer meta data: {}", data.meta_data);
    println!("innter data: {}", data.inner_data.data);
    // data implements AsRef<str> so it can be used as such
    process_as_string(data);
}

pub struct ByteData {
    meta_data: i32,
    inner_data: InnerData,
}

pub struct InnerData {
    data: String,
}

impl AsRef<str> for ByteData {
    fn as_ref(&self) -> &str {
        &self.inner_data.data
    }
}

// This function accepts ANY type (T) that implements the AsRef<str> trait.
fn process_as_string(data: impl AsRef<str>) {
    // We call the required method: as_ref()
    println!("Processed AsRef: '{}')", data.as_ref());
}
