pub(crate) fn main() {
    let dog = Dog {};
    let cat = Cat {};

    //dog.make_sound();
    //cat.make_sound();

    do_animal_stuff(dog);
}

// using generics - static dispatch version
fn do_animal_stuff<Animal: AnimalSound + AnimalEating>(a: Animal) {
    a.make_sound();
    a.eat_food();
}

struct Cat {}
struct Dog {}

trait AnimalEating {
    fn eat_food(&self);
}
trait AnimalSound {
    fn make_sound(&self);
}

impl AnimalEating for Cat {
    fn eat_food(&self) {
        println!("cat eating cat food.");
    }
}
impl AnimalSound for Cat {
    fn make_sound(&self) {
        println!("cat meowing");
    }
}

impl AnimalEating for Dog {
    fn eat_food(&self) {
        println!("dog eating dog food");
    }
}
impl AnimalSound for Dog {
    fn make_sound(&self) {
        println!("dog barking.");
    }
}
