fn main() {
    let print_add = |x, y| println!("Print from a closure: {}", x + y);
    let a = 42;
    let b = 100;
    print_add(a, b);

    let add = |one, two| {
        println!("adding one and two");
        one + two
    };
    let more = add(42, 100) + 100;

    println!("complex calc: {}", more);

    // mutating inside closure
    struct Person {
        name: String,
    }
    let mut p1 = Person {
        name: "Raphael".to_string(),
    };
    println!("name: {}", p1.name);
    let mut change_name = || p1.name = "Jack".to_string();
    change_name();
    println!("name: {}", p1.name);
}
