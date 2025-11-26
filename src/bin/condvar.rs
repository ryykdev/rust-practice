use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    // Inside of our lock, spawn a new thread, and then wait for it to start.
    thread::spawn(move || {
        println!("spawn thread");
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });

    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("wait on condvar");
        started = cvar.wait(started).unwrap();
    }
}
