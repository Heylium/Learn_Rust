use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::option::Option::Some;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Message>>>)->Worker{
        let thread=thread::spawn(move|| loop {
            let message=receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job)=>{println!("Worker {} receive a job",id);job();},
                Message::Terminate=>{println!("Worker {} receive terminate",id);break;},
            }
        });
        Worker{
            id,
            thread:Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,

    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool{
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);
        let mut workers = Vec::with_capacity(size);
        let (sender,receiver)=mpsc::channel();
        let receiver=Arc::new(Mutex::new(receiver));    //加锁，保证线程安全

        for id in 0..size {
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool{workers,sender}
    }
}


impl Drop for ThreadPool{
    fn drop(&mut self){
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some (thread)=worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}
