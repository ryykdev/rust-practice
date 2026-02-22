fn main() {
    let s = "hello world";
    // must_use tells user to use the result
    result(s);
}

#[must_use]
fn result(s: &str) -> String {
    println!("print s: {s}");
    format!("{}{}", s, " from earth")
}
