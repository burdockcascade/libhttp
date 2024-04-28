use std::sync::{Arc, Mutex};
use log::debug;
use simplelog::SimpleLogger;
use libhttp::http::{CONTENT_TYPE_TEXT_PLAIN, HEADER_CONTENT_TYPE, Status};
use libhttp::message::{HttpRequest, HttpResponse};
use libhttp::server::{HttpHandler, HttpServer};

fn main() {

    let _ = SimpleLogger::init(log::LevelFilter::Debug, simplelog::Config::default());

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

        debug!("Request: {}", request);

        self.counter += 1;
        
        let body = format!("Hello, world! {}", self.counter).as_bytes().to_vec();

        return HttpResponse::builder()
            .status(Status::Ok)
            .header(HEADER_CONTENT_TYPE, CONTENT_TYPE_TEXT_PLAIN)
            .body(body)
            .build()
    }
}