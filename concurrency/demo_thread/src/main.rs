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

    handler.join().unwrap(); // even when the main thread get's over, the function will still run until the spawned thread is done
}
