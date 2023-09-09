use std::str;
use std::str::Utf8Error;
use super::method::{Method, MethodError};
use std::convert:: TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Formatter, Display, Debug};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

        let request = str::from_utf8(buf)?;

        // Variable shadowing is going on here:
        /**
         * Variable shadowing in Rust is the practice of 
         * declaring a new variable with the same name as an existing variable
         * within a narrower scope, temporarily hiding the outer variable.
         */
        let (method, request)=get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request)=get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _)=get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidRequest);
        }

        let method: Method = method.parse()?;

        let mut query_string: Option<&str> = None;

        // match path.find('?'){
        //     Some(i) => {
        //         query_string = Some(&path[i+1..]);
        //         path = &path[i+1..];
        //     },
        //     None => {}
        // }

        // let q = path.find('?');
        // if q.is_some(){
        //     let i = q.unwrap();
        //     query_string = Some(&path[i+1..]);
        //     path = &path[i+1..];
        // }

        // if let implementation reason
        if let Some(i) = path.find('?'){
            query_string = Some(&path[i+1..]);
            path = &path[i+1..];
        }

        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {

    for (i,c) in request.chars().enumerate(){
        if c == ' ' || c == '\r'{
            return Some((&request[..i],&request[i+1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidProtocol,
    InvalidEncoding,
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str{
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

/*
Doing this so we can write this code snippet 
`let request = str::from_utf8(buf)?;` instead of
`let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?`
*/
impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError{
    fn from(_: MethodError) -> Self{
        Self::InvalidMethod
    }
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError{

}