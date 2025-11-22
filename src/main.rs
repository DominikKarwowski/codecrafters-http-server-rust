mod endpoints;

use codecrafters_http_server::model::HttpRequest;
use codecrafters_http_server::ThreadPool;
use std::env;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

fn main() {
    println!("Logs from your program will appear here!");

    let srv_settings = Arc::new(read_settings_or_default());
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let srv_settings = Arc::clone(&srv_settings);

        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream, srv_settings);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn try_read_settings() -> Option<ServerSettings> {
    let args: Vec<String> = env::args().collect();

    let directory = if args.get(1)? == "--directory" {
        args.get(2)?.to_string()
    } else {
        return None;
    };

    Some(ServerSettings { directory })
}

fn read_settings_or_default() -> ServerSettings {
    try_read_settings().unwrap_or_else(|| ServerSettings::create_default())
}

fn handle_connection(mut stream: TcpStream, srv_settings: Arc<ServerSettings>) {
    println!("accepted new connection");

    let buf_reader = BufReader::new(&stream);

    let raw_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let http_request = HttpRequest::deserialize(raw_request);

    let http_response = endpoints::handle(http_request, srv_settings);

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

struct ServerSettings {
    directory: String,
}

impl ServerSettings {
    const fn create_default() -> ServerSettings {
        ServerSettings {
            directory: String::new(),
        }
    }
}
