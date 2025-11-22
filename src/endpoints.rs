use codecrafters_http_server::model::{HttpMethod, HttpRequest, HttpResponse, HttpResponseStatus};
use std::collections::HashMap;
use std::error::Error as AnyError;
use std::fs::File;
use std::{fs, io};
use std::io::{Error, Read};
use std::path::PathBuf;
use std::sync::Arc;
use futures::AsyncWriteExt;
use codecrafters_http_server::header_keys::{CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use codecrafters_http_server::media_types::{application, text};
use crate::ServerSettings;

pub fn handle(request: HttpRequest, env: Arc<ServerSettings>) -> HttpResponse {
    match request.path.as_str() {
        s if s.starts_with("/echo/") => echo_get(&request),
        s if s.starts_with("/files/") => match request.http_method {
            HttpMethod::Get => files_get(&request, &env),
            HttpMethod::Post => files_post(&request, &env),
        },
        "/user-agent" => user_agent_get(&request),
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
        (String::from(CONTENT_TYPE), String::from(text::PLAIN)),
        (String::from(CONTENT_LENGTH), format!("{}", body.len())),
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
                    String::from(CONTENT_TYPE),
                    String::from(application::OCTET_STREAM),
                ),
                (String::from(CONTENT_LENGTH), format!("{}", content.len())),
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

fn read_file(path: &PathBuf) -> Result<Vec<u8>, io::Error> {
    let mut file_content_buf = Vec::new();

    File::open(path)?.read_to_end(&mut file_content_buf)?;

    Ok(file_content_buf)
}

fn user_agent_get(request: &HttpRequest) -> HttpResponse {
    let status = HttpResponseStatus::Ok;
    let status_line = get_response_status_line(&status);

    let body = match request.headers.get(USER_AGENT) {
        Some(v) => v.as_bytes().to_vec(),
        None => Vec::new(),
    };

    let headers = HashMap::from([
        (String::from(CONTENT_TYPE), String::from("text/plain")),
        (String::from(CONTENT_LENGTH), format!("{}", body.len())),
    ]);

    HttpResponse {
        status,
        status_line,
        headers,
        body,
    }
}

fn files_post(request: &HttpRequest, env: &ServerSettings) -> HttpResponse {
    let filename = String::from(&request.path[7..]); // move to get()?
    let file_path = PathBuf::from(&env.directory).join(filename);

    // TODO: optional - not in requirements - return 415 UnsupportedMediaType
    if !request.headers.contains_key(CONTENT_TYPE)
        || request.headers[CONTENT_TYPE] != application::OCTET_STREAM
    {
        panic!("Unspecified or unhandled media type")
    }

    match write_file(&file_path, &request.headers[CONTENT_LENGTH], &request.body) {
        Ok(_) => {
            let status = HttpResponseStatus::Created;
            let status_line = get_response_status_line(&status);

            HttpResponse {
                status,
                status_line,
                headers: HashMap::new(),
                body: Vec::new(),
            }
        }
        Err(e) => {
            // TODO: optional - not in requirements - return 500 InternalServerError
            panic!("Failed to create file: {}", e)
        }
    }
}

fn write_file(
    file_path: &PathBuf,
    content_len: &str,
    body: &Vec<u8>,
) -> Result<(), Box<dyn AnyError>> {
    let file_len = content_len.parse()?;

    let mut file = File::create(file_path)?.set_len(file_len)?;

    fs::write(file_path, body)?;

    // let w = file.write_all(body);

    Ok(())
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
        HttpResponseStatus::Created => "201 Created",
        HttpResponseStatus::NotFound => "404 Not Found",
    };

    format!("HTTP/1.1 {response_status}")
}
