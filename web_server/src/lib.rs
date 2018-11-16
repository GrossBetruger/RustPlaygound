use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;


pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Sender<Message>
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>
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

enum Message {
    NewJob(Job),
    Terminate
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                            println!("Worker {} got a job; executing.", id);
                            job.call_box();
                    }
                    Message::Terminate => {
                         println!("Worker {} was told to terminate.", id);
                         break;
                    }
                }
            }
        });

        Worker {
            id: id,
            handle: Some(thread)
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

            self.sender.send(Message::NewJob(job)).unwrap();

        }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("sending terminate signal to all threads");

        for _ in &self.threads {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.threads {
            println!("shutting down worker: {}", worker.id);
            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
                println!("successfully shut down worker: {}", worker.id)
            }
        }
    }
}
