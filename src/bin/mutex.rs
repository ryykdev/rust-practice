use std::{
    ops::AddAssign,
    panic,
    sync::Mutex,
    thread::{self, Scope},
    time::Duration,
};

fn main() {
    let score = Mutex::new(0u16);
    //let unlocked_data = score.lock();
    //let mut data = unlocked_data.unwrap();
    //data.add_assign(42);
    //
    //println!("data: {}", data);
    //drop(data);

    let myfunc = || {
        println!("thread 1 is waiting for the mutex lock...");
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
            println!("thread 1 is adding {i}");
            thread::sleep(Duration::from_millis(300));
        }
    };
    let myfunc2 = || loop {
        println!("thread 2 is waiting for the mutex lock");
        let guard = score.try_lock();

        if guard.is_ok() {
            let mut data = guard.unwrap();
            for i in 1..10 {
                data.add_assign(i);
                println!("thread 2 is adding {i}");
            }
            break;
        }
        thread::sleep(Duration::from_millis(300));
    };

    thread::scope(|s: &Scope<'_, '_>| {
        let handle1 = s.spawn(myfunc);
        let handle2 = s.spawn(myfunc2);
    });

    println!("main thread total score: {}", score.lock().unwrap());
}
