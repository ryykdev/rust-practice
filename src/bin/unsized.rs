fn main() {
    bar(&["Ryyk", "Dev"]);
    bar(&["String".to_string(), "Second String".to_string()]);
    bar(&["&str", "String Mixed".to_string()]);
}

pub trait Hei {
    fn hei(&self);
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }
}
impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

// compiler looks at type of "Ryyk" and its traits
// and calls hei()
pub fn foo() {
    "Ryyk".hei();
}

// this gets monomorphized
pub fn hei_generic(h: impl Hei) {
    h.hei();
}
// like this
pub fn bar_str(h: &str) {
    h.hei();
}
// but just Hei wont work because its
// not sized, the types can be any size which
// implement the trait Hei
/*
pub fn take_hei(h: Hei) {
    h.hei();
}
*/

/*
// dyn Hei is not Sized
// anything that implements Hei can be used
// all of them are different sizes - expl: like give me
// the forth element - but Rust doesn't know where that is
// in momory
pub fn bar(s: &[dyn Hei]) {
    for h in s {
        h.hei();
    }
}
*/

// trait object only behaves as some underling trait
