fn main() {
    let byte_data = ByteData::from("hello world".to_string());
    let s: String = byte_data.into();

    println!("{}", s);
}

struct ByteData {
    inner_data: String,
}

impl From<String> for ByteData {
    fn from(value: String) -> Self {
        ByteData { inner_data: value }
    }
}

// preferred, automatically gives Into()
// due to the relationship
impl From<ByteData> for String {
    fn from(byte_data: ByteData) -> String {
        byte_data.inner_data
    }
}

//impl Into<String> for ByteData {
//    fn into(self) -> String {
//        self.inner_data
//    }
//}
