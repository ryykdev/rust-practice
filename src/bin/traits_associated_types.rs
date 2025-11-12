fn main() {
    // associated types
    let my_struct = MyStruct { data: 42 };

    println!("{}", my_struct.data);

    println!("{}", my_struct.get_bar());
}
trait Foo {
    type Bar<'a>
    where
        Self: 'a;

    fn get_bar<'a>(&'a self) -> Self::Bar<'a>;
}

impl Foo for MyStruct {
    type Bar<'a> = &'a i32;

    fn get_bar<'a>(&'a self) -> Self::Bar<'a> {
        &self.data
    }
}

struct MyStruct {
    data: i32,
}
