#![feature(try_blocks)]
use core::{
    error::Error,
    result::Result::{self, Err},
};
fn main() {
    // try_block example
    let r = do_the_thing();
    match r {
        Ok(sum) => println!("try succeeded with sum={sum}"),
        Err(e) => println!("Error with: {e}"),
    }
}

fn do_the_thing() -> Result<u8, Box<dyn Error>> {
    // do something that needs cleanup
    let a: &mut u8 = &mut 1;
    let b: &mut u8 = &mut 2;

    let mut thing = Thing::setup(a, b);
    let r = try { thing.sum()? };
    thing.cleanup()?;
    println!("thing cleanup a={} and b={}", thing.a, thing.b);

    r
}

struct Thing<'a> {
    pub a: &'a mut u8,
    pub b: &'a mut u8,
}

impl<'a> Thing<'a> {
    fn setup(a: &'a mut u8, b: &'a mut u8) -> Self {
        println!("setup Thing with {} and {}", a, b);
        Thing { a, b }
    }
    fn sum(&self) -> Result<u8, Box<dyn Error>> {
        println!("try sum operation..");
        Err("Something seriously went wrong")?;
        Ok(*self.a + *self.b)
    }
    fn cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        *self.a = 0;
        *self.b = 0;
        Ok(())
    }
}
