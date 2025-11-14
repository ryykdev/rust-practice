use std::fmt::Debug;
fn main() {
    let string = String::from("Strings are Sized");

    let str = "string slices are not Sized";

    process_sized(&string);
    process_sized(str);

    process_unsized(&string);
    process_unsized(str);
}

fn process_sized<T: Debug>(item: T) {
    println!("Sized item {:?}", item);
}

fn process_unsized<T: Debug + ?Sized>(item: &T) {
    println!("Maybe unsized item {:?}", item);
}
