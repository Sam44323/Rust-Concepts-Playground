use std::sync::Mutex;

fn main() {
    /*
     * Mutex uses locking system to ensure that only one thread can access the data at a time. When accessing, it locks the data during tapping and when done, it unlocks the data.
     */

    let m = Mutex::new(5);
}
