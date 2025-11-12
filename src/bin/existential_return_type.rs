fn main() {
    let shape = Circle { radius: 8 };
    return_shape(shape);
}

fn return_shape(shape: impl Shape) -> impl Shape {
    shape.print_shape();
    shape
}

trait Shape {
    fn print_shape(&self) {}
}

struct Circle {
    radius: i32,
}

struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl Shape for Circle {
    fn print_shape(&self) {
        println!("circle with radius: {}", self.radius);
    }
}

impl Shape for Triangle {
    fn print_shape(&self) {
        println!("triagle with {}x{}x{}", self.a, self.b, self.c);
    }
}
