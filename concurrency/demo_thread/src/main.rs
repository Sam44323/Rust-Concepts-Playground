use std::{thread, time::Duration};

fn main() {
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main  thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handler.join().unwrap(); // will block the main thread from finishing, until the thread attached with the handler finishes
}
