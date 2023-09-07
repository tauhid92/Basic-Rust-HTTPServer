use super::method::Method;
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

impl Request {
    pub fn process_request(self) -> String{
        "".to_string()
    }
}