fn main() {
    let s = "hello world";
    // must_use tells user to use the result
    must_use_result(s);
}

#[must_use]
fn must_use_result(s: &str) -> String {
    format!("{}{}", s, " from earth")
}
