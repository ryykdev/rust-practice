pub(crate) fn main() {
    let dog = Dog {};
    let cat = Cat {};

    make_animal_sound(&dog);
    make_animal_sound(&cat);
}

// using dynamic dispatch a fat pointer is created.
// the first word is pointing to the objects data
// the second word is pointing to a vTable.
// during run-time rust looks up those pointers
// and calls the appropiate function through the
// vTable. This way there is only one make_animal_sound
// function that gets called - contrasting this with
// static dispatch, where for each generic variant
// a function gets generated.
fn make_animal_sound(a: &dyn AnimalSound) {
    a.make_sound();
}

struct Animal {}

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
