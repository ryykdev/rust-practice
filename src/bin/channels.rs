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
                    break;
                }
            }
        }
    };
    let mut handles = Vec::new();
    for thread_nr in 0..50 {
        let transmitter_clone = transmitter.clone();
        let send_thread = std::thread::spawn({
            move || {
                for x in 1..=6 {
                    thread::sleep(Duration::from_millis(1000));
                    let send_result = transmitter_clone.send(x);
                    println!(">> thread {thread_nr} send status: {}", send_result.is_ok());
                }
            }
        });
        handles.push(send_thread);
    }

    let receive_thread = std::thread::spawn(processor);

    loop {
        let mut all_done = true;
        if !receive_thread.is_finished() {
            all_done = false;
        }
        for handle in &handles {
            if !handle.is_finished() {
                all_done = false;
            }
        }
        if all_done {
            println!("lets get out of here!");
            break;
        }
    }
}
