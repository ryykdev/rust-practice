use std::{
    sync::mpsc::{self},
    thread,
    time::Duration,
};

fn main() {
    let (transmitter, reciever) = mpsc::channel();

    let _ = transmitter.send(42);
    let _ = transmitter.send(100);

    println!("Recieved {}", reciever.recv().unwrap());
    println!("Recieved {}", reciever.recv().unwrap());
    // recv hangs without messages
    //println!("will this hang?");
    //println!("Recieved {}", reciever.recv().unwrap());

    let processor = move || {
        println!("starting processor");
        let mut fails = 0;
        loop {
            println!(">> attempting to recieve message from channel ...");
            let result = reciever.recv_timeout(Duration::from_millis(800));
            if result.is_ok() {
                println!(">>>> recieved: {}", result.unwrap());
                fails = 0;
            } else {
                println!(">>>> nothing in channel");
                fails += 1;
                println!(">>>> fails: {fails}");
                if fails > 10 {
                    std::process::exit(0);
                }
            }
        }
    };
    let transmitter_clone = transmitter.clone();
    let send_thread = std::thread::spawn({
        move || {
            for x in 1..=6 {
                thread::sleep(Duration::from_millis(1000));
                let send_result = transmitter_clone.send(x);
                println!(">> send status: {}", send_result.is_ok());
            }
        }
    });
    let send_thread_2 = std::thread::spawn(move || {
        for x in 100..=106 {
            thread::sleep(Duration::from_millis(1000));
            let send_result = transmitter.send(x);
            println!(">> send status 2: {}", send_result.is_ok());
        }
    });

    let receive_thread = std::thread::spawn(processor);

    send_thread.join();
    send_thread_2.join();
    receive_thread.join();
}
