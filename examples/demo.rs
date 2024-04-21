use std::sync::{Arc, Mutex};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TerminalMode, TermLogger};
use libhttp::http::{Header, Status};
use libhttp::message::{HttpRequest, HttpResponse};
use libhttp::server::{HttpHandler, HttpServer};

fn main() {

    let _ = TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto
    );

    HttpServer::new()
        .handler(Arc::new(Mutex::new(MyHandler { counter: 0 })))
        .start()
        .unwrap();

}

struct MyHandler {
    counter: u32,
}

impl HttpHandler for MyHandler {
    fn handle(&mut self, request: &HttpRequest) -> HttpResponse {
        
        let headers = vec![
            Header { key: "Content-Type".to_string(), value: "text/plain".to_string() },
            Header { key: "Server".to_string(), value: "teapot".to_string() },
        ];
        
        let body = Some(format!("Hello, world! {}", self.counter).as_bytes().to_vec());
        
        self.counter += 1;
        
        let response = HttpResponse::new(Status::Ok, headers, body);
        response
    }
}