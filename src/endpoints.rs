use crate::model::{HttpRequest, HttpResponse, HttpResponseStatus};

pub fn handle(request: HttpRequest) -> HttpResponse {
    match request.path.as_str() {
        s if s.starts_with("/echo/") => echo_get(&request),
        "/" => index_get(),
        _ => not_found(),
    }
}

fn index_get() -> HttpResponse {
    let status = HttpResponseStatus::Ok;
    let status_line = get_response_status_line(&status);

    HttpResponse {
        status,
        status_line,
        headers: Vec::new(),
        body: String::new(),
    }
}

fn echo_get(request: &HttpRequest) -> HttpResponse {
    let status = HttpResponseStatus::Ok;
    let status_line = get_response_status_line(&status);

    let body = String::from(&request.path[6..]);

    let headers = vec![
        String::from("Content-Type: text/plain"),
        format!("Content-Length: {}", body.len()),
    ];

    HttpResponse {
        status,
        status_line,
        headers,
        body,
    }
}

fn not_found() -> HttpResponse {
    let status = HttpResponseStatus::NotFound;
    let status_line = get_response_status_line(&status);

    HttpResponse {
        status,
        status_line,
        headers: Vec::new(),
        body: String::new(),
    }
}

fn get_response_status_line(http_response: &HttpResponseStatus) -> String {
    let response_status = match http_response {
        HttpResponseStatus::Ok => "200 OK",
        HttpResponseStatus::NotFound => "404 Not Found",
    };

    String::from(format!("HTTP/1.1 {response_status}\r\n\r\n"))
}
