use std::thread;

pub struct ThreadPool{
    threads: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id , thread}
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            // Storing the threads in the vector
            threads.push(Worker::new(id));
        }
        ThreadPool {threads}
    }

    pub fn execute<F>(&self, f: F)
        where
            F:FnOnce() + Send + 'static
    {
    }
}
