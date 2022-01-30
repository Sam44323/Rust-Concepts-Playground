use std::thread;

/**
 * Use of JoinHandler struct
 * An owned permission to join on a thread (block on its termination). A JoinHandle detaches the associated thread when it is dropped, which means that there is no longer any handle to thread and no way to join on it.
 */

pub struct ThreadPool {
  threads: Vec<thread::JoinHandle<()>>,
}

// this struct will store the id of the thread which will be doing the work and the thread that waits for listening and handling the requests

pub struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl ThreadPool {
  /**
   * Create a new ThreadPool.
   *
   * The size is the number of threads in the pool.
   *
   * # Panics
   *
   * The `new` function will panic if the size is zero.
   */
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let mut threads = Vec::with_capacity(size);

    for _ in 0..size {
      // create threads
    }
    ThreadPool { threads }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
  }
}
