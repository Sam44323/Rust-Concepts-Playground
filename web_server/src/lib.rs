use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/**
 * Use of JoinHandler struct
 *
 * An owned permission to join on a thread (block on its termination). A JoinHandle detaches the associated thread when it is dropped, which means that there is no longer any handle to thread and no way to join on it.
 */

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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

    let mut workers = Vec::with_capacity(size);

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver)); // as we want our threads to share the receivers that can also be mutated/updated, so we use Arc<Mutex<receiver>>

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
  }
}

// this struct will store the id of the thread which will be doing the work and the thread that waits for listening and handling the requests

pub struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(|| {
      receiver;
    });
    Worker { id, thread }
  }
}
