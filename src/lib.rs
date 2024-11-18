use std::{
    //mpsc is for the communication channel
    //The Arc type let multiple workers own the receiver
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool
{
    //Vector of all workers and the sender of a communication channel
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
//What happens when creating a thread pool (like an init)
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        //EValuates that its a valid size before creating the pool
        assert!(size > 0);

        //Declares the sender and receiver of a communication channel
        let (sender, receiver) = mpsc::channel();

        //Make the receiver Arc type to being able to use it with different workers
        let receiver = Arc::new(Mutex::new(receiver));

        //Creates a vector the previous size
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            //Adding each worker to the vector
            //Sending its id and reciver from the channel
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers, 
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

struct Worker 
{
    id: usize,
    //**
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker
{
    //The Mutec<T> ensures that only onw worker thread at a time is requesting a job
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker
    {
        let thread = thread::spawn(move || loop{
            //lock is called to acquire the mutex
            //recv is called to receive a job from the channel
            //With recv if there is no job yet, the thread will wait unlit it is
            let message = receiver.lock().unwrap().recv();

            match message 
            {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
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

impl Drop for ThreadPool
{
    fn drop(&mut self)
    {
        //Stop receiving messages
        drop(self.sender.take()); //Closes the channel
        //Loop through each thread pool
        for worker in &mut self.workers
        {
            println!("Shutting down worker {}", worker.id);

            //Destructure Some and get the thread to be able to join it
            if let Some(thread) = worker.thread.take()
            {
                //Let each worker finish its current task
                thread.join().unwrap();
            }
            
        }
    }
}