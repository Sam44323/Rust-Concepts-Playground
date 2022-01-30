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
  sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
  NewJob(Job),
  Terminate,
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
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap(); // sending the job to one of the threads in the pool
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.workers {
      println!("Shutting down worker {}!", worker.id);

      if let Some(thread) = worker.thread.take()
      // taking the join handler out and replacing it with None
      {
        thread.join().unwrap(); // waiting for the thread to complete their job before shut-down
      }
    }
  }
}

// this struct will store the id of the thread which will be doing the work and the thread that waits for listening and handling the requests

pub struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap(); // calling lock to get the the mutex and then calling recv to get the job from the receiver
      match job {
        Message::NewJob(job) => {
          println!("Worker {} got a job; executing.", id);
          job();
        }
        Message::Terminate => {
          println!("Worker {} was told to terminate", id);
          break;
        }
      }
    });
    Worker {
      id,
      thread: Some(thread),
    }
  }
}
