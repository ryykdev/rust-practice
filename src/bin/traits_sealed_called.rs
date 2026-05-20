mod traits_sealed;
use core::cmp::Ord;

use traits_sealed::Speed;
fn main() {
       // 
    let u32 = 100u32;
    // downstream we can call methods of the sealed trait
    let speed = u32.max_speed();
    println!("print {u32} speed: {speed}");
    // but we can implement traits on new types
    impl Speed for usize {
    fn max_speed(&self) -> u32 {
        *self as u32  
    }
}
    let usize = 200usize;
    let speed = usize.max_speed();
    println!("print {usize} speed: {speed}");
}

// everyone can implement your pub trait 
// if you update your traits methods all 
// the downstream code breaks

