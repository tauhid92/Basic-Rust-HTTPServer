
use std::{fmt::{Result as FmtResult, Formatter, Display, Debug},
        net::TcpStream,
        io::{Write, Result as IoResult},
    };

use super::StatusCode;

#[derive(Debug)]
pub struct Response{
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self{
        Response {status_code, body}
    }

    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()>{
        let body = match &self.body{
            Some(b) => b,
            None => "",
        };
        
        write!(f, "HTTP/1.1 {} {}\r\n\r\n{}", 
        self.status_code,
        self.status_code.reason_phrase(),
        body
        )
    }
}