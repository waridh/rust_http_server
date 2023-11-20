use std::{
    sync::{mpsc, Arc, Mutex},
    thread};

pub struct ThreadPool{
    workers: Vec<Worker>,
    tx: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/**
 * A thread. Has to recive a job to start doing work.
 */
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>> ) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = rx.lock().unwrap().recv();
            match job {
                Ok(x) => {
                    println!("Worker {id} got a job; Executing.");
                    x();
                },
                Err(_) => {
                    println!("Worker {id} found an error.");
                    break;
                }
            }
        });
        Worker { id , thread: Some(thread)}
    }
}

/**
 * The implementation of ThreadPool. This is the front-end of this module.
 * Created to 
 */
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        let (tx, receiver) = mpsc::channel();
        let rx = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            // Storing the threads in the vector
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }
        ThreadPool {workers, tx:Some(tx)}
    }

    pub fn execute<F>(&self, f: F)
        where
            F:FnOnce() + Send + 'static
    {
        let job = Box::new(f);      // Wrapping up the closure for channel
        self.tx.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.tx.take());
        for worker in &mut self.workers {
            let id = worker.id;
            println!("Shutting down worker: {id}");

            if let Some(x) = worker.thread.take() {
                x.join().unwrap();
            }
        }
    }
}
