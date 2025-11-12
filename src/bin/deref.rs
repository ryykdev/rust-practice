fn main() {
    // Deref and AsRef
    let wrapped = Wrapper {
        outside: "hi",
        inside: Inside { value: 42 },
    };
    println!("outside {}, inside {}", wrapped.outside, wrapped.value);
}

struct Wrapper<T, U> {
    outside: T,
    inside: Inside<U>,
}

struct Inside<T> {
    value: T,
}

impl<T, U> std::ops::Deref for Wrapper<T, U> {
    // we want to Wrapper to behave like the Inside struct
    type Target = Inside<U>;

    fn deref(&self) -> &Self::Target {
        &self.inside
    }
}
