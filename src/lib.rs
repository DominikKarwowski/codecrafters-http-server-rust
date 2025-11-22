pub mod header_keys;
pub mod media_types;
pub mod model;

use crate::model::{HttpRequest, HttpResponse};
use std::env::Args;
use std::io::Write;
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn start_http_server<F>(settings: ServerSettings, endpoint_handler: F)
where
    F: Fn(HttpRequest, Arc<ServerSettings>) -> HttpResponse + Send + Sync + 'static,
{
    let listener = TcpListener::bind(settings.get_addr()).unwrap();
    let pool = ThreadPool::new(settings.threads_count);
    let settings = Arc::new(settings);
    let endpoint_handler = Arc::new(endpoint_handler);

    for stream in listener.incoming() {
        let settings = Arc::clone(&settings);
        let endpoint_handler = Arc::clone(&endpoint_handler);

        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream, settings, endpoint_handler);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection<F>(mut stream: TcpStream, settings: Arc<ServerSettings>, endpoint_handler: Arc<F>)
where
    F: Fn(HttpRequest, Arc<ServerSettings>) -> HttpResponse + Send + Sync + 'static,
{
    println!("accepted new connection");

    let http_request = HttpRequest::from_stream(&stream);
    let http_response = endpoint_handler(http_request, settings);
    let result = stream.write_all(&http_response.serialize());

    match result {
        Ok(_) => {
            println!("response successfully written to TCP stream");
        }
        Err(e) => {
            println!("failed to write response to TCP stream: {}", e)
        }
    }
}

pub struct ServerSettings {
    pub root_dir: String,
    pub host_addr: Ipv4Addr,
    pub port: u16,
    pub threads_count: usize,
}

impl ServerSettings {
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host_addr, self.port)
    }

    pub fn from_env_args(args: Args) -> ServerSettings {
        ServerSettings::try_read_settings(args).unwrap_or_else(|| ServerSettings::create_default())
    }

    fn create_default() -> ServerSettings {
        ServerSettings {
            root_dir: String::new(),
            host_addr: Ipv4Addr::new(127, 0, 0, 1),
            port: 4221,
            threads_count: 4,
        }
    }

    fn try_read_settings(args: Args) -> Option<ServerSettings> {
        let args: Vec<String> = args.collect();

        let root_dir = if args.get(1)? == "--directory" {
            args.get(2)?.to_string()
        } else {
            return None;
        };

        Some(ServerSettings {
            root_dir,
            host_addr: Ipv4Addr::new(127, 0, 0, 1),
            port: 4221,
            threads_count: 4,
        })
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });

        Worker { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is a number of threads in a pool.
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

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
