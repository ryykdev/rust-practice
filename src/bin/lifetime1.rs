struct First<'a> {
    image: &'a Vec<i32>,
}

fn main() {
    let image = vec![1, 2, 3];

    let first: First<'_> = First { image: &image };

    //    drop(image);

    process_first(first);
}

fn process_first(first: First) {
    // let x = first.image.pop().unwrap();
    // let y = first.image.pop().unwrap();
    println!("{}", first.image.first().unwrap());
}
