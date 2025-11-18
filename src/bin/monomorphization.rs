fn main() {
    todo!();
}

// generic function gets converted for all concrete
// types it needs to handle
fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

// function with refstr
fn strlen_refstr(s: &str) -> usize {
    s.len()
}

// function with String - because string implements
// AsRef<str>
fn strlen_string(s: String) -> usize {
    s.len()
}
