use std::str;
use std::str::Utf8Error;
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
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buf){
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)){
        //     Ok(request) => {},
        //     Err(e) => return Err(e),
        // }

        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        // All the code above can be abridged to this line however you need to implement a trait From<Utf8Error>
        // '?' is basically a declarative macro
        let request = str::from_utf8(buf)?;

        unimplemented!()
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