use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    Workers: Vec<Worker>,
    sender: mpsc::Sender<job>,
}

//
type job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    ///Creates a new threadpool
    ///
    /// size must be greater than 0
    ///
    /// if not its panic and crash and burn
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        //get the send and reciever from the channel
        //the channel is way for multiple threads to talk to each other
        //the sender will be own by the threadpool and the reciever will be owned by the workers
        let (sender, receiver) = mpsc::channel();

        //make the reciver muatble and have multiple owners
        let receiver = Arc::new(Mutex::new(receiver));

        //Make a vec with the capcicty of size
        ///this is where the threads/workers are suppose to go
        let mut Workers = Vec::with_capacity(size);

        //creating and pushing the worker thread into Workers Vec 
        for id in 0..size {
            Workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        //putting the Workers Vec and sender to the ThreadPool struct
        ThreadPool { Workers, sender }
    }

    //F is a genric type
    //generic type is a placeholder type that has a blueprint Aka traits
    /// executes a closure in a thread
    /// 
    pub fn execute<F>(&self, f: F)
    //where is for traits
    //since F is generic type it doesn't really have a data type so it can lead to issues and errors
    //to prevent this we have traits which are basically rules or tests that F has to follow/past
    //if it pasts these tests then whatever type F is it will be able run with errors
    where
        //These are the rules F has to follow
        //F is a closure that takes in no arguments and returns nothing
        F: FnOnce() + Send + 'static,
    {
        //wrap the function/jobin a box
        let job = Box::new(f);
        //send the job to the channel AKA mpsc::channel();
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

//this is a trait that is used to define the job that the worker will do
impl Worker {
    //this is a constructor for the worker
    //it takes in the id of the worker and the receiver of the threadpool
    //the receiver is a mutex that is used to send jobs to the worker
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<job>>>) -> Worker {
        //Since this a new function and it will be called owce
        //we need execute the code in a new thread
        //to do this  we made let job a closure that will loop even if new is not called
        //the worker will keep looping and executing the job during the lifetime of the threadpool even tho threadpool won't be
        // called again
        let thread = thread::spawn(move || loop {
            //this waits for a job to be sent to the worker
            //the job is a boxed function that is sent to the worker
            //the worker will execute the function and then send a message back to the threadpool
            let Job = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();

            println!("Worker {} got a job executing", id);

            //execute the job if the job is received
            //the job is a boxed function that is sent to the worker
            //the job is handle || handle_connection(stream) closure form the main function
            Job();
        });

        //return the worker struct
        Worker { id, thread }
    }
}


