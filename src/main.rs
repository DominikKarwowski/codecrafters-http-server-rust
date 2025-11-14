use std::io::{prelude::*};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("accepted new connection");

    let mut request = String::new();

    if let Ok(_) = stream.read_to_string(&mut request) {
        println!("{request}");

        let response = match parse_request(&request) {
            HttpResponse::Ok => "HTTP/1.1 200 OK\r\n\r\n",
            HttpResponse::BadRequest => "HTTP/1.1 404 Not Found\r\n\r\n",
        };

        let result = stream.write_all(response.as_bytes());

        match result {
            Ok(_) => {
                println!("response successfully written to TCP stream");
            }
            Err(_) => {
                println!("failed to write response to TCP stream")
            }
        }
    }
}

fn parse_request(request: &str) -> HttpResponse {
    let split: Vec<&str> = request.split_whitespace().collect();

    let request_path = split[1];

    match request_path {
        "/" => HttpResponse::Ok,
        _ => HttpResponse::BadRequest,
    }
}

enum HttpResponse {
    Ok,
    BadRequest,
}