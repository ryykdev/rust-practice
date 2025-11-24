fn main() {
    let x = 42;
    let x_ref = &x;

    println!("{:p}", x_ref);

    let y = 1.0;

    // mismatched types expected `&{integer}`
    //x_ref = &y;
}
