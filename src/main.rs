use std::io::{prelude::*, BufReader};
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

    let buf_reader = BufReader::new(&stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let path = get_request_path(http_request[0].as_str());

    let http_response = match path {
        "/" => HttpResponse::Ok,
        _ => HttpResponse::BadRequest,
    };

    let response_status_line = match http_response {
        HttpResponse::Ok => "HTTP/1.1 200 OK\r\n\r\n",
        HttpResponse::BadRequest => "HTTP/1.1 404 Not Found\r\n\r\n",
    };

    let result = stream.write_all(response_status_line.as_bytes());

    match result {
        Ok(_) => {
            println!("response successfully written to TCP stream");
        }
        Err(e) => {
            println!("failed to write response to TCP stream: {}", e)
        }
    }
}

fn get_request_path(request_line: &str) -> &str {
    let lines: Vec<_> = request_line.split_whitespace().collect();
    lines[1]
}

enum HttpResponse {
    Ok,
    BadRequest,
}
