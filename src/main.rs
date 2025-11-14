#[allow(unused_imports)]
use std::io::{BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";

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