use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::net::TcpListener;
use std::io::{Read, Write};


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response;
}
pub struct Server {
    address : String
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self, mut handler: impl Handler){
        println!("Server listening at port: {}",self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Request::try_from(&buffer as &[u8]); or
                            let response = match Request::try_from(&buffer[..]){
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&request)
                            };

                            if let Err(e) = response.send(&mut stream){
                                println!("Failed to send request")
                            }
                        },
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("Failed to establish connection: {}", e);
                }
            }
        }
       
        
    }
}