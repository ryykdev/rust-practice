fn main() {
    let mut vec_string = vec!["Hello".to_string(), "world!".to_string()];

    let first = vec_string.iter_mut().next().unwrap();

    println!("{}", first);

    first.push_str("ooohh..");

    dbg!(vec_string);
}
