use crate::model::{HttpRequest, HttpResponse, HttpResponseStatus};
use crate::ServerSettings;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;
use std::sync::Arc;

pub fn handle(request: HttpRequest, env: Arc<ServerSettings>) -> HttpResponse {
    match request.path.as_str() {
        s if s.starts_with("/echo/") => echo_get(&request),
        s if s.starts_with("/files/") => files_get(&request, &env),
        "/user-agent" => user_agent(&request),
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
        headers: HashMap::new(),
        body: Vec::new(),
    }
}

fn echo_get(request: &HttpRequest) -> HttpResponse {
    let status = HttpResponseStatus::Ok;
    let status_line = get_response_status_line(&status);

    let body = request.path[6..].as_bytes().to_vec();

    let headers = HashMap::from([
        (String::from("Content-Type"), String::from("text/plain")),
        (String::from("Content-Length"), format!("{}", body.len())),
    ]);

    HttpResponse {
        status,
        status_line,
        headers,
        body,
    }
}

fn files_get(request: &HttpRequest, env: &ServerSettings) -> HttpResponse {
    let filename = String::from(&request.path[7..]); // move to get()?
    let file_path = PathBuf::from(&env.directory).join(filename);

    match read_file(&file_path) {
        Ok(content) => {
            let status = HttpResponseStatus::Ok;
            let status_line = get_response_status_line(&status);
            let headers = HashMap::from([
                (
                    String::from("Content-Type"),
                    String::from("application/octet-stream"),
                ),
                (String::from("Content-Length"), format!("{}", content.len())),
            ]);
            let body = content;
            HttpResponse {
                status,
                status_line,
                headers,
                body,
            }
        }
        Err(_) => {
            let status = HttpResponseStatus::NotFound;
            let status_line = get_response_status_line(&status);

            HttpResponse {
                status,
                status_line,
                headers: HashMap::new(),
                body: Vec::new(),
            }
        }
    }
}

fn read_file(path: &PathBuf) -> Result<Vec<u8>, Error> {
    let mut file_content_buf = Vec::new();

    File::open(path)?.read_to_end(&mut file_content_buf)?;

    Ok(file_content_buf)
}

fn user_agent(request: &HttpRequest) -> HttpResponse {
    let status = HttpResponseStatus::Ok;
    let status_line = get_response_status_line(&status);

    let body = match request.headers.get("User-Agent") {
        Some(v) => v.as_bytes().to_vec(),
        None => Vec::new(),
    };

    let headers = HashMap::from([
        (String::from("Content-Type"), String::from("text/plain")),
        (String::from("Content-Length"), format!("{}", body.len())),
    ]);

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
        headers: HashMap::new(),
        body: Vec::new(),
    }
}

fn get_response_status_line(http_response: &HttpResponseStatus) -> String {
    let response_status = match http_response {
        HttpResponseStatus::Ok => "200 OK",
        HttpResponseStatus::NotFound => "404 Not Found",
    };

    format!("HTTP/1.1 {response_status}")
}
