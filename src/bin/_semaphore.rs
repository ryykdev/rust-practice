fn main() {
    println!("--- Semaphore Demonstration: Printer Queue (Max 3 Concurrent Jobs) ---");

    // Initialize the semaphore with a limit of 3 permits.
    let semaphore = Arc::new(Semaphore::new(3));
    let mut handles = vec![];

    // Create 5 threads (print jobs)
    for i in 1..=5 {
        let semaphore_clone = Arc::clone(&semaphore);
        let handle = thread::spawn(move || {
            let job_id = i;
            println!("[Job {}] Waiting for an available printer...", job_id);

            // 1. Acquire the permit (P-operation)
            semaphore_clone.acquire();

            // *** CRITICAL SECTION (Simulating printing) ***
            println!("[Job {}] 🖨️ Printing in progress...", job_id);
            // Simulate work time
            thread::sleep(Duration::from_millis(500));
            println!("[Job {}] ✅ Finished printing.", job_id);
            // **********************************************

            // 2. Release the permit (V-operation)
            semaphore_clone.release();
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("--- All print jobs complete. ---");
}

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

// A simple counting semaphore implemented using standard library primitives.
struct Semaphore {
    // The current count of available permits.
    count: Mutex<usize>,
    // Used to signal waiting threads when a permit becomes available.
    condvar: Condvar,
    // The maximum capacity (total permits).
    capacity: usize,
}
impl Semaphore {
    fn new(capacity: usize) -> Semaphore {
        Semaphore {
            count: Mutex::new(capacity),
            condvar: Condvar::new(),
            capacity,
        }
    }

    fn acquire(&self) {
        let mut count = self.count.lock().unwrap();

        // Wait as long as the count is zero (no permits available).
        // This wait handles spurious wakeups by checking the condition in a loop.
        while *count == 0 {
            count = self.condvar.wait(count).unwrap();
        }

        // Decrement the count, consuming a permit.
        *count -= 1;
        println!("  -> Thread acquired permit. Remaining permits: {}", *count);
    }

    fn release(&self) {
        let mut count = self.count.lock().unwrap();

        // Ensure we don't exceed the initial capacity (optional, but good practice).
        if *count < self.capacity {
            // Increment the count, releasing a permit.
            *count += 1;
            println!("  <- Thread released permit. Remaining permits: {}", *count);
        }

        // Notify one waiting thread that a permit is available.
        self.condvar.notify_one();
    }
}
