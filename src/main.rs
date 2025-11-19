mod endpoints;
mod model;

use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use codecrafters_http_server::ThreadPool;
use crate::model::HttpRequest;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("accepted new connection");

    let buf_reader = BufReader::new(&stream);

    let raw_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let http_request = HttpRequest::deserialize(raw_request);

    let http_response = endpoints::handle(http_request);

    let result = stream.write_all(http_response.serialize().as_bytes());

    match result {
        Ok(_) => {
            println!("response successfully written to TCP stream");
        }
        Err(e) => {
            println!("failed to write response to TCP stream: {}", e)
        }
    }
}
