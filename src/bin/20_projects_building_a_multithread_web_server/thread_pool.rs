use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::{JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job),
    Terminate,
}
/// 自定义线程池
pub struct ThreadPool {
    workers: Vec<Worker>,
    /// 任务队列
    sender: Option<Sender<Message>>
}
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Self {workers, sender: Some(sender)}
    }
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        if self.sender.as_ref().unwrap().send(Message::NewJob(job)).is_err() {
            println!("send job failed...");
        }
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 关闭sender后，将关闭对应的channel
        if let Some(Sender) = self.sender.take() {
            println!("Sending terminate message to all workers.");
            for _ in &mut self.workers {
                Sender.send(Message::Terminate).unwrap();
            }
        }
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
struct Worker{
    id: usize,
    thread: Option<JoinHandle<()>>
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        // 这里的循环不能用while let(还包括 if let 和 match)
/*
        let thread = thread::spawn(move || {
            while let Ok(msg) = receiver.lock().unwrap().recv() {
                match msg {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing...", id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate...", id);
                        break;
                    }
                }
            } //锁直到这里才被释放
        });
*/
        let thread = thread::spawn(move || {
            loop {
                let msg = receiver.lock().unwrap().recv().unwrap();
                /* //这样写也不行
                let receiver = receiver.lock().expect("failed to get a lock...");
                let msg = receiver.recv().expect("failed to recv a msg...");
                */
                match msg {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing...", id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate...", id);
                        break;
                    }
                }
            }
        });

        // 每个 `Worker` 都拥有自己的唯一 id
        Worker{id, thread: Some(thread)}
    }
}


