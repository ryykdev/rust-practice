//struct First<'a> {
//    image: &'a Vec<i32>,
//}
//
//fn lifetime1() {
//    let image = vec![1, 2, 3];
//
//    let first = First { image: &image };
//
//    drop(image);
//
//    process_first(first);
//}
//
//fn process_first(first: First) {
//    let x = first.image.take(0);
//    let y = first.image.take(1);
//
//    println!("{} and {}", x, y);
//}
