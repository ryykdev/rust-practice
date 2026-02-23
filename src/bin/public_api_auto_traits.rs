use std::rc::Rc;

fn main() {
    // traits for apis should implement auto-traits like
    // Sized, Send, Sync, Unpin so they can be used in a
    // multi-threading context
    // the Rc pointer is not Send and therefore not thread-safe
    let hold_int = HoldInt {
        number: 8,
        _not_send: Rc::new(()),
    };
    println!("holds int: {}", hold_int.number);
}

pub struct HoldInt {
    number: u8,
    _not_send: Rc<()>,
}

#[cfg(test)]
mod test {
    use crate::HoldInt;
    fn impl_auto_traits<T: Sized + Send + Sync + Unpin>() {}
    #[test]
    // this test must merely compile for the auto-traits check
    fn test_auto_traits_ok() {
        impl_auto_traits::<HoldInt>();
    }
}
