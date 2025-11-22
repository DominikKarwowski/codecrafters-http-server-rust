use std::collections::HashMap;
use crate::header_keys::{CONTENT_LENGTH, CONTENT_TYPE};
use crate::model::{HttpResponse, HttpResponseStatus};

pub struct HttpResponseContent<'a> {
    pub body: Vec<u8>,
    pub media_type: &'a str,
}

pub fn ok(content: Option<HttpResponseContent>) -> HttpResponse {
    let status = HttpResponseStatus::Ok;
    let status_line = get_response_status_line(&status);

    let (headers, body) = match content {
        Some(content) => {
            let headers = HashMap::from([
                (String::from(CONTENT_TYPE), String::from(content.media_type)),
                (String::from(CONTENT_LENGTH), format!("{}", content.body.len())),
            ]);
            (headers, content.body)
        }
        None => (HashMap::new(), Vec::new())
    };

    HttpResponse {
        status,
        status_line,
        headers,
        body,
    }
}

pub fn created() -> HttpResponse {
    let status = HttpResponseStatus::Created;
    let status_line = get_response_status_line(&status);

    HttpResponse {
        status,
        status_line,
        headers: HashMap::new(),
        body: Vec::new(),
    }
}

pub fn not_found() -> HttpResponse {
    let status = HttpResponseStatus::NotFound;
    let status_line = get_response_status_line(&status);

    HttpResponse {
        status,
        status_line,
        headers: HashMap::new(),
        body: Vec::new(),
    }
}

fn get_response_status_line(http_response: &HttpResponseStatus) -> String {
    let response_status = match http_response {
        HttpResponseStatus::Ok => "200 OK",
        HttpResponseStatus::Created => "201 Created",
        HttpResponseStatus::NotFound => "404 Not Found",
    };

    format!("HTTP/1.1 {response_status}")
}