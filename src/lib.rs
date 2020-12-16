use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};
use std::{
    collections::HashMap,
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    sync::mpsc::{channel, Receiver, Sender},
};
use std::{io::prelude::*, rc::Rc};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let (sender, receiver) = channel();
            let mut workers = Vec::with_capacity(size);
            let receiver = Arc::new(Mutex::new(receiver));
            for id in 0..size {
                let worker = Worker::new(id, Arc::clone(&receiver));
                workers.push(worker);
            }
            Ok(ThreadPool { workers, sender })
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap()
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct PoolCreationError;

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        Worker {
            id,
            thread: Some(spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} got a termination message; killing.", id);
                        break;
                    }
                }
            })),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job),
    Terminate,
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Method {
    POST,
    PUT,
    GET,
    DELETE,
}
#[derive(Debug, Clone)]
pub struct Controller(pub fn(TcpStream) -> ());
pub struct Route {
    pub method: Method,
    pub path: String,
    pub controller: Controller,
}
pub struct JApp {
    routes: HashMap<Method, Vec<(String, Controller)>>,
    address: SocketAddr,
    thread_amt: usize,
}
pub fn startie(routes: Vec<Route>, address: SocketAddr, thread_amt: usize) -> () {
    let mut routeMap: HashMap<Method, Vec<(String, Controller)>> = HashMap::new();
    routes
        .into_iter()
        .for_each(|r| match routeMap.get_mut(&r.method) {
            Some(stuff) => {
                stuff.push((r.path, r.controller));
            }
            None => {
                let new_one = vec![(r.path, r.controller)];
                routeMap.insert(r.method, new_one);
            }
        });

    let listener = TcpListener::bind(address).unwrap();
    let pool = ThreadPool::new(thread_amt).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let lines: Vec<String> = buffer.lines().map(|l| l.unwrap().to_string()).collect();
        let mut first_line = lines.get(0).unwrap().split(" ");
        let verb = first_line.next().unwrap();
        let path = first_line.next().unwrap();
        let controller = match verb {
            "POST" => routeMap.get(&Method::POST).unwrap().into_iter().find(|p| {
                let mut pattern = String::from("/");
                pattern.push_str(p.0.as_str());
                return path == pattern;
            }),
            "PUT" => routeMap.get(&Method::PUT).unwrap().into_iter().find(|p| {
                let mut pattern = String::from("/");
                pattern.push_str(p.0.as_str());
                return path == pattern;
            }),
            "GET" => routeMap.get(&Method::GET).unwrap().into_iter().find(|p| {
                let mut pattern = String::from("/");
                pattern.push_str(p.0.as_str());
                return path == pattern;
            }),
            "DELETE" => routeMap
                .get(&Method::DELETE)
                .unwrap()
                .into_iter()
                .find(|p| {
                    let mut pattern = String::from("/");
                    pattern.push_str(p.0.as_str());
                    return path == pattern;
                }),
            _ => routeMap.get(&Method::GET).unwrap().into_iter().find(|p| {
                let mut pattern = String::from("/");
                pattern.push_str(p.0.as_str());
                return path == pattern;
            }),
        };
        let thang = controller.unwrap().to_owned().1;
        pool.execute(move || handle_conn(stream, thang));
    }
}

fn handle_conn(mut stream: TcpStream, controller: Controller) -> () {
    controller.0(stream);
}
fn default_controller(mut stream: TcpStream) {
    stream.flush().unwrap();
}
pub fn create_response_line(code: usize, msg: &str) -> String {
    format!("HTTP/1.1 {} {}\r\n\r\n", code, msg)
}
pub fn create_response(response_line: & str, response: & str) -> String {
    format!("{}{}", response_line, response)
}
