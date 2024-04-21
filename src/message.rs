use std::fmt::Display;
use crate::http::{CONNECTION_KEEP_ALIVE, Header, HEADER_CONNECTION, HEADER_CONTENT_LENGTH, HEADER_DATE, HEADER_SERVER, HTTP_VERSION_1_1, Method, Status};

pub struct HttpRequest {
    hostname: String,
    path: String,
    method: Method,
    pub headers: Vec<Header>,
    pub body: Option<Vec<u8>>,
}

impl HttpRequest {
    pub fn parse(request_line: String) -> Self {

        let mut parts = request_line.trim().split_whitespace();
        let method = Method::from_str(parts.next().unwrap()).unwrap();
        let path = parts.next().unwrap().to_string();
        let _ = parts.next().unwrap().to_string();
        
        HttpRequest {
            hostname: String::new(),
            method,
            path,
            headers: Vec::new(),
            body: None,
        }
    }
}

impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.method, self.path, self.hostname)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HttpResponse {
    pub status: Status,
    headers: Vec<Header>,
    body: Option<Vec<u8>>,
}

impl HttpResponse {
    pub fn new(status: Status, headers: Vec<Header>, body: Option<Vec<u8>>) -> Self {
        
        let content_length = match &body {
            Some(body) => body.len(),
            None => 0,
        };
        
        let mut response_headers = vec![
            Header::new(HEADER_SERVER, "Rust Server"),
            Header::new(HEADER_DATE, &chrono::Utc::now().to_rfc2822()),
            Header::new(HEADER_CONTENT_LENGTH, &content_length.to_string()),
            Header::new(HEADER_CONNECTION, CONNECTION_KEEP_ALIVE),
        ];
        
        response_headers.extend(headers);
        
        HttpResponse {
            status,
            headers: response_headers,
            body,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        // write status line
        buffer.extend(format!("{} {}\r\n", HTTP_VERSION_1_1, self.status.as_u16()).as_bytes().to_vec());

        // write headers
        for header in &self.headers {
            buffer.extend(format!("{}: {}\r\n", header.key, header.value).as_bytes().to_vec());
        }

        // write a blank line to separate headers from body
        buffer.extend("\r\n".as_bytes().to_vec());

        // write body
        if let Some(body) = &self.body {
            buffer.extend(body.to_vec());
        }
        
        buffer
    }
}