use super::http::{Request, Response, StatusCode};
use super::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler{
    fn handle_request(&mut self, request: &Request) -> Response {
        todo!()
    }
}

