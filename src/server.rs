use std::sync::{Arc, Mutex};
use async_std::io;
use async_std::io::{BufReader, BufWriter};
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use log::{debug, info, trace};
use crate::http::{Header, HEADER_CONTENT_LENGTH, Method, Status};
use crate::message::{HttpRequest, HttpResponse};

pub trait HttpHandler: Send + Sync + 'static {
    fn handle(&mut self, request: &HttpRequest) -> HttpResponse;
}

pub struct HttpServer {
    handler: Option<Arc<Mutex<dyn HttpHandler>>>,
}

impl HttpServer {
    pub fn new() -> Self {
        HttpServer {
            handler: None,
        }
    }

    pub fn start(&self) -> io::Result<()> {
        task::block_on(async {
            let listener = TcpListener::bind("127.0.0.1:80").await?;
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