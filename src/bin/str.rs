fn main() {
    // let mut str = "zzz";
    // println!("mut str zzz: {:p}", str);
    // // in-place mutation of the str not possible
    // //*str = "ferris";
    // // but creating a new &[str] and replacing the pointer is okay
    // str = "aaaa";
    // println!("str as aaa: {:p}", str);

    // let vec_str = vec![str];
    // println!("vec ref: {:p}", &vec_str);
    // println!("vec as_pointer: {:p}", vec_str.as_ptr());
    // println!("vec first unwrap: {:p}", vec_str.first().unwrap());
    // // &str
    // println!("deref vec[0]: {:p}", *vec_str.first().unwrap());
    // // i32 slice
    // let y = [32i32, 51, 62];
    // println!("i32 array: {:p}", &y);
    // let z = &y[1..2];
    // println!("array slice: {:p}", z);
    // testing Box pointers
    // let heap = Box::new(5);
    // println!("{:p}", heap);
    // println!("{}", *heap);

    // let heap = Box::new("hello world");
    // println!("{:p}", heap);
    // println!("{:p}", *heap);
    // testing take()

    let  s = Box::new(42);
    let mut t = Box::new(1);
    let x = std::mem::replace(&mut t, s);
    // s = x;
    println!("{}", x);
    println!("{}", t);
}
