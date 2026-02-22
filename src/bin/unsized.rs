fn main() {
    todo!();
}

pub trait Hei {
    fn hei(&self);
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

pub fn foo() {
    "Ryyk".hei();
}

pub fn bar(h: impl Hei) {
    h.hei();
}
