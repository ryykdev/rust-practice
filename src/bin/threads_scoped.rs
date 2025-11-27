use std::thread::{self, Scope};

//  Unlike non-scoped threads, scoped threads can borrow non-`'static` data,
//  as the scope guarantees all threads will be joined at the end of the scope.
//
//  All threads spawned within the scope that haven't been manually joined
//  will be automatically joined before this function returns.
fn main() {
    let mut age = 42;
    let mut person01 = Person {
        name: "Ryyk".to_string(),
    };

    let print_age = || {
        age += 100;
        person01.name = "Mutanted Ryyk".to_string();
        println!("closure: Your age is {}", age);
        println!("closure: Your name is {}", &person01.name);
    };

    //thread::spawn(print_age).join();

    thread::scope(|scope: &Scope<'_, '_>| {
        // join() is optional, gives access to a result
        let _result = scope.spawn(print_age).join();
    });

    person01.name = "Origianal Ryyk".to_string();
    println!("Your age is {age}");
    println!("Your name is {}", &person01.name);

    println!("fin");
}

struct Person {
    name: String,
}
