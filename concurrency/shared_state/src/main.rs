use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    /*
     * Mutex uses locking system to ensure that only one thread can access the data at a time. When accessing, it locks the data during tapping and when done, it unlocks the data.
     */

    // Arc allows you to safely transfer ownership of data between threads

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..11 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Value for counter: {}", *counter.lock().unwrap());
}
