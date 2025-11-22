use codecrafters_http_server::header_keys::{CONTENT_TYPE, USER_AGENT};
use codecrafters_http_server::http_response::HttpResponseContent;
use codecrafters_http_server::media_types::{application, text};
use codecrafters_http_server::model::{HttpMethod, HttpRequest, HttpResponse};
use codecrafters_http_server::{http_response, ServerSettings};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use std::{fs, io};

type AnyError = Box<dyn std::error::Error>;

pub fn handler(request: HttpRequest, env: Arc<ServerSettings>) -> HttpResponse {
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
    http_response::ok(None)
}

fn echo_get(request: &HttpRequest) -> HttpResponse {
    let body = request.path[6..].as_bytes().to_vec();

    http_response::ok(Some(HttpResponseContent {
        body,
        media_type: text::PLAIN,
    }))
}

fn files_get(request: &HttpRequest, env: &ServerSettings) -> HttpResponse {
    let filename = String::from(&request.path[7..]);
    let file_path = PathBuf::from(&env.root_dir).join(filename);

    match read_file(&file_path) {
        Ok(body) => {
            http_response::ok(Some(HttpResponseContent{
                body,
                media_type: application::OCTET_STREAM,
            }))
        }
        Err(_) => {
            http_response::not_found()
        }
    }
}

fn read_file(path: &PathBuf) -> Result<Vec<u8>, io::Error> {
    let mut file_content_buf = Vec::new();

    File::open(path)?.read_to_end(&mut file_content_buf)?;

    Ok(file_content_buf)
}

fn user_agent_get(request: &HttpRequest) -> HttpResponse {
    let body = match request.headers.get(USER_AGENT) {
        Some(v) => v.as_bytes().to_vec(),
        None => Vec::new(),
    };

    http_response::ok(Some(HttpResponseContent{
        body,
        media_type: text::PLAIN,
    }))
}

fn files_post(request: &HttpRequest, env: &ServerSettings) -> HttpResponse {
    let filename = String::from(&request.path[7..]);
    let file_path = PathBuf::from(&env.root_dir).join(filename);

    // TODO: optional - not in requirements - return 415 UnsupportedMediaType
    if !request.headers.contains_key(CONTENT_TYPE)
        || request.headers[CONTENT_TYPE] != application::OCTET_STREAM
    {
        panic!("Unspecified or unhandled media type")
    }

    match write_file(&file_path, &request.body) {
        Ok(_) => {
            http_response::created()
        }
        Err(e) => {
            // TODO: optional - not in requirements - return 500 InternalServerError
            panic!("Failed to create file: {}", e)
        }
    }
}

fn write_file(file_path: &PathBuf, body: &Vec<u8>) -> Result<(), AnyError> {
    fs::write(file_path, body)?;
    Ok(())
}

fn not_found() -> HttpResponse {
    http_response::not_found()
}
