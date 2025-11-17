use std::collections::HashMap;

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
    pub body: String,
}

impl HttpRequest {
    pub fn deserialize(raw_request: Vec<String>) -> HttpRequest {
        let request_line: Vec<_> = raw_request[0].split_whitespace().collect();
        let headers = HttpRequest::parse_headers(&raw_request[1..]);
        let body = String::new();

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

    fn parse_headers(headers: &[String]) -> HashMap<String, String> {
        let mut headers_map = HashMap::new();

        for h in headers {
            let kvp: Vec<&str> = h.split(": ").collect();
            // TODO: handle error case with malformed headers
            headers_map.insert(String::from(kvp[0]), String::from(kvp[1]));
        };

        headers_map
    }
}

pub enum HttpResponseStatus {
    Ok,
    NotFound,
}

pub struct HttpResponse {
    pub status: HttpResponseStatus,
    pub status_line: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn serialize(&self) -> String {
        let mut raw_response = String::new();

        raw_response.push_str(&self.status_line);
        raw_response.push_str("\r\n");

        for (key, value) in &self.headers {
            raw_response.push_str(key);
            raw_response.push_str(": ");
            raw_response.push_str(value);
            raw_response.push_str("\r\n");
        }
        raw_response.push_str("\r\n");

        raw_response.push_str(&self.body);

        raw_response
    }
}
