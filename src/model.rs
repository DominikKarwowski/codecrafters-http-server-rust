use crate::header_keys::CONTENT_LENGTH;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

pub enum HttpMethod {
    Get,
    Post,
}

impl HttpMethod {
    fn from_str(input: &str) -> Result<HttpMethod, String> {
        match input {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            s => Err(format!("Unknown method {}", s)),
        }
    }
}

pub struct HttpRequest {
    pub http_method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn from_stream(stream: &TcpStream) -> HttpRequest {
        let mut buf_reader = BufReader::new(stream);

        let mut request_pre_body: Vec<String> = Vec::new();

        loop {
            let mut line = String::new();
            // TODO: optional - not in requirements - return relevant error
            buf_reader.read_line(&mut line).unwrap();

            let line = line.trim_end_matches(&['\r', '\n'][..]).to_string();
            if line.is_empty() {
                break;
            }

            request_pre_body.push(line);
        }

        let request_line: Vec<_> = request_pre_body[0].split_whitespace().collect();
        let headers = HttpRequest::parse_headers(&request_pre_body[1..]);

        let body = match headers.get(CONTENT_LENGTH) {
            Some(len) => {
                // TODO: optional - not in requirements - return 400 BadRequest
                let len: usize = len.parse().expect(format!("Invalid value in the {CONTENT_LENGTH} header (expected a non-negative integer)").as_str());
                let mut content = vec![0; len];
                // TODO: optional - not in requirements - return 400 BadRequest
                buf_reader.read_exact(&mut content).expect("Failed to read body of a length specified in a header");
                content
            }
            None => Vec::new(),
        };

        // TODO: optional - not in requirements - handle error scenario with 500
        let method = HttpMethod::from_str(request_line[0]).unwrap();
        let path = String::from(request_line[1]);

        HttpRequest {
            http_method: method,
            path,
            headers,
            body,
        }
    }

    fn parse_headers(headers: &[String]) -> HashMap<String, String> {
        let mut headers_map = HashMap::new();

        for h in headers {
            let kvp: Vec<&str> = h.split(": ").collect();
            // TODO: handle error case with malformed headers
            headers_map.insert(String::from(kvp[0]), String::from(kvp[1]));
        }

        headers_map
    }
}

pub enum HttpResponseStatus {
    Ok,
    Created,
    NotFound,
}

pub struct HttpResponse {
    pub status: HttpResponseStatus,
    pub status_line: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn serialize(&self) -> Vec<u8> {
        let mut raw_response = Vec::new();

        raw_response.extend_from_slice(self.status_line.as_bytes());
        raw_response.extend_from_slice("\r\n".as_bytes());

        for (key, value) in &self.headers {
            raw_response.extend_from_slice(key.as_bytes());
            raw_response.extend_from_slice(": ".as_bytes());
            raw_response.extend_from_slice(value.as_bytes());
            raw_response.extend_from_slice("\r\n".as_bytes());
        }
        raw_response.extend_from_slice("\r\n".as_bytes());

        raw_response.extend_from_slice(&self.body);

        raw_response
    }
}
