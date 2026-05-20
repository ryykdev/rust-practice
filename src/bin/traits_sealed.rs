fn main() {}

pub trait Speed: private::Sealed {
    fn max_speed(&self) -> u32;
}

impl Speed for u32 {
    fn max_speed(&self) -> u32 {
        *self
    }
}

mod private {
    // rapper for sealed
    pub trait Sealed {}

    impl Sealed for u32 {}
}
