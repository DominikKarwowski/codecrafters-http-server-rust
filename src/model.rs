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
    pub headers: Vec<String>, // TODO: model as HashMap
    pub body: String,
}

impl HttpRequest {
    pub fn serialize(raw_request: Vec<String>) -> HttpRequest {
        let request_line: Vec<_> = raw_request[0].split_whitespace().collect();
        let headers = raw_request[1]
            .split_whitespace()
            .map(str::to_string)
            .collect();
        let body = String::clone(&raw_request[2]);

        // TODO: handle error scenario with 500
        let method = HttpMethod::from_str(request_line[0]).unwrap();
        let path = String::from(request_line[1]);

        HttpRequest {
            http_method: method,
            path,
            headers,
            body,
        }
    }
}

pub enum HttpResponseStatus {
    Ok,
    NotFound,
}

pub struct HttpResponse {
    pub status: HttpResponseStatus,
    pub status_line: String,
    pub headers: Vec<String>, // TODO: model as HashMap
    pub body: String,
}

impl HttpResponse {
    pub fn deserialize(&self) -> String {
        let mut raw_response = String::new();

        raw_response.push_str(&self.status_line);
        raw_response.push_str("\r\n");

        for s in &self.headers {
            raw_response.push_str(s);
            raw_response.push_str("\r\n");
        }
        raw_response.push_str("\r\n");

        raw_response.push_str(&self.body);

        raw_response
    }
}
