use std::str::FromStr;

#[derive(Debug)]
pub enum Method{
    GET, POST, PUT, DELETE, PATCH, OPTIONS, CONNECT, HEAD, TRACE
}


impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "PATCH" => Ok(Self::PATCH),
            "OPTIONS" => Ok(Self::OPTIONS),
            "CONNECT" => Ok(Self::CONNECT),
            "HEAD" => Ok(Self::HEAD),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(MethodError),
            
        }
    }
}

 pub struct MethodError;