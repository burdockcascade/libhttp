use std::hash::DefaultHasher;
use std::sync::{Arc, Mutex};
use async_std::io;
use async_std::io::{BufReader, BufWriter};
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use log::{debug, info, trace};
use crate::http::{CONNECTION_KEEP_ALIVE, Header, HEADER_CONNECTION, HEADER_CONTENT_LENGTH, HEADER_SERVER, Method, Status};
use crate::message::{HttpRequest, HttpResponse};

const DEFAULT_SERVER_NAME: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub trait HttpHandler: Send + Sync + 'static {
    fn handle(&mut self, request: &HttpRequest) -> HttpResponse;
}

#[derive(Clone)]
pub struct HttpServer {
    hostname: String,
    port: u16,
    default_headers: Vec<Header>,
    handler: Option<Arc<Mutex<dyn HttpHandler>>>,
}

impl Default for HttpServer {
    fn default() -> Self {
        HttpServer {
            hostname: "127.0.0.1".to_string(),
            port: 80,
            default_headers: vec![
                Header::new(HEADER_SERVER, DEFAULT_SERVER_NAME),
                Header::new(HEADER_CONNECTION, CONNECTION_KEEP_ALIVE),
            ],
            handler: None,
        }
    }
}

impl HttpServer {
    pub fn new(hostname: String, port: u16, default_headers: Vec<Header>, handler: Option<Arc<Mutex<dyn HttpHandler>>>) -> Self {
        HttpServer {
            hostname,
            port,
            handler,
            default_headers
        }
    }

    pub fn builder() -> HttpServerBuilder {
        HttpServerBuilder::default()
    }

    pub fn start(&self) -> io::Result<()> {

        let address = format!("{}:{}", self.hostname, self.port);

        task::block_on(async {
            let listener = TcpListener::bind(address).await?;
            debug!("Listening on {}", listener.local_addr()?);

            let mut incoming = listener.incoming();

            while let Some(stream) = incoming.next().await {
                let stream = stream?;
                let handler = self.handler.clone();
                task::spawn(async {
                    Self::handle_connection(stream, handler).await.unwrap();
                });
            }
            Ok(())
        })
    }

    pub fn handler(&mut self, handler: Arc<Mutex<dyn HttpHandler>>) -> &mut Self {
        self.handler = Some(handler);
        self
    }

    async fn handle_connection(stream: TcpStream, handler: Option<Arc<Mutex<dyn HttpHandler>>>) -> io::Result<()> {

        debug!("Incoming connection from: {}", stream.peer_addr()?);
        
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        loop {

            // Read HTTP request line
            let mut request_line = String::new();
            reader.read_line(&mut request_line).await?;

            if request_line.is_empty() {
                break;
            }

            let mut request = HttpRequest::parse(request_line);

            // Parse headers
            loop {
                let mut header_line = String::new();
                reader.read_line(&mut header_line).await?;

                // Check if the header line is empty
                if header_line.trim().is_empty() {
                    break;
                }
                
                request.headers.push(Header::parse(header_line));
            }

            // Check if the request has a Content-Length header
            let content_length = request.headers.iter()
                .find(|header| header.key() == HEADER_CONTENT_LENGTH)
                .and_then(|header| header.value().parse::<usize>().ok());

            // If the request has a body, read it
            if let Some(content_length) = content_length {
                // Read the message body
                let mut message_body = vec![0u8; content_length];
                reader.read_exact(&mut message_body).await?;
                request.body = Some(message_body);
            }
            
            let response = match handler {
                Some(ref handler) => handler.lock().unwrap().handle(&request),
                None => HttpResponse::new(Status::BadRequest, vec![], None),
            };
            
            writer.write_all(&response.to_bytes()).await?;
            
            writer.flush().await?;

        }

        Ok(())
    }
}

pub struct HttpServerBuilder {
    server: HttpServer,
}

impl Default for HttpServerBuilder {
    fn default() -> Self {
        HttpServerBuilder {
            server: HttpServer::default(),
        }
    }
}

impl HttpServerBuilder {

    pub fn hostname<H>(&mut self, hostname: H) -> &mut Self where H: Into<String>{
        self.server.hostname = hostname.into();
        self
    }

    pub fn port(&mut self, port: u16) -> &mut Self {
        self.server.port = port;
        self
    }

    pub fn default_headers(&mut self, default_headers: Vec<Header>) -> &mut Self {
        self.server.default_headers = default_headers;
        self
    }

    pub fn handler(&mut self, handler: Arc<Mutex<dyn HttpHandler>>) -> &mut Self {
        self.server.handler = Some(handler);
        self
    }

    pub fn build(&self) -> HttpServer {
        self.server.clone()
    }

    pub fn start(&self) -> io::Result<()> {
        self.server.start()
    }

}