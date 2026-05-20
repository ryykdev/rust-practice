fn main() {
    // supertraits
    let jet = Jet {};
    jet.take_off();
    println!("jet flying at: {} km/h", jet.top_speed());
}

trait CanFly {
    fn take_off(&self);
}

trait TopSpeed: CanFly {
    fn top_speed(&self) -> i32;
}

pub struct Jet;

// TopSpeed requires the CanFly trait bound
// without it the code won't compile
impl CanFly for Jet {
    fn take_off(&self) {
        println!("jet take off!");
    }
}

impl TopSpeed for Jet {
    fn top_speed(&self) -> i32 {
        2000
    }
}
