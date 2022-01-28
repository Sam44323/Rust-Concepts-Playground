use std::sync::mpsc;
use std::{thread, time::Duration};

/**
 * MPSC means, multi producer and single consumer. It means a channel can have multiple senders and one receiver.
 */

fn main() {
    let (sender, receiver) = mpsc::channel();
    let another_sender = sender.clone(); // cloning the sender to another sender with same receiver but for another thread

    /*
     * we are using move so that sender is scoped to the closure, cause passing senders between threads can be a security risk. If the receiving end is dropped due to some reason, the unwrap will panic and throw and error
     */

    thread::spawn(move || {
        let msg_list: Vec<String> = vec![
            String::from("hello"),
            String::from("world"),
            String::from("!"),
        ];
        for msg in msg_list {
            sender.send(msg).unwrap(); // send will take ownership of msg
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let msg_list: Vec<String> = vec![
            String::from("message"),
            String::from("from"),
            String::from("another"),
            String::from("sender"),
        ];
        for msg in msg_list {
            another_sender.send(msg).unwrap(); // send will take ownership of msg
            thread::sleep(Duration::from_millis(1));
        }
    });

    // treating the receiver as an iterator for receiving message over every iteration from the sender
    for rcv in receiver {
        println!("{}", rcv);
    }

    // let received = receiver.recv().expect("The receiver is offline!"); // recv will block the main thread execution while returning a result whereas the try_recv will return a result in immediate manner
}
