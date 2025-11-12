fn main() {
    let type_with = Foo {
        wave_hi: 1.0,
        rap_in_bar: Bar { t: 5 },
    };

    println!("{}", type_with.rap_in_bar.t);
    println!("{}", type_with.wave_hi);
}

struct Foo<T, U> {
    wave_hi: T,
    rap_in_bar: Bar<U>,
}

struct Bar<T> {
    t: T,
}
