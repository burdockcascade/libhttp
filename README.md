# libhttp

## Description
a lightweight low-level http server library for rust

## Usage
```rust
use std::sync::{Arc, Mutex};
use libhttp::http::{CONTENT_TYPE_TEXT_PLAIN, HEADER_CONTENT_TYPE, Status};
use libhttp::message::{HttpRequest, HttpResponse};
use libhttp::server::{HttpHandler, HttpServer};

fn main() {
    HttpServer::builder()
        .hostname("127.0.0.1")
        .port(80)
        .handler(Arc::new(Mutex::new(MyHandler { counter: 0 })))
        .start()
        .expect("server to run successfully")
}

struct MyHandler {
    counter: u32,
}

impl HttpHandler for MyHandler {
    fn handle(&mut self, request: &HttpRequest) -> HttpResponse {

        self.counter += 1;
        
        let body = format!("Hello, world! {}", self.counter).as_bytes().to_vec();

        return HttpResponse::builder()
            .status(Status::Ok)
            .header(HEADER_CONTENT_TYPE, CONTENT_TYPE_TEXT_PLAIN)
            .body(body)
            .build()
    }
}
```