use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;


pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Sender<Job>
}

struct Worker {
    id: usize,
    handel: thread::JoinHandle<()>
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl <F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id: id,
            handel: thread
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = vec![];

        for id in 0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)))
        }
        ThreadPool {
            threads: threads,
            sender: sender
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
        {
            let job = Box::new(f);

            self.sender.send(job).unwrap();

        }

}
