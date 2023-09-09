use std::str;
use super::method::Method;
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
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match str::from_utf8(buf){
            Ok(request) => {},
            Err(_) => {}
        }
        unimplemented!()
        //comment
    }
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