use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::net::TcpListener;
use std::io::{Read, Write};

pub struct Server {
    address : String
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self){
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
                            match Request::try_from(&buffer[..]){
                                Ok(request) => {
                                    dbg!(request);
                                    let response = Response::new(StatusCode::Ok,Some("<h1>Hello! Hello!</h1>".to_string()));
                                    write!(stream, "{}", response);
                                },
                                Err(e) => {
                                    println!("Failed to parse request.")
                                }
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