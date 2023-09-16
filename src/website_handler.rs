use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;

pub struct WebsiteHandler{
    public_path: String,
}

impl WebsiteHandler{
    pub fn new(public_path: String) -> Self{
        Self{ public_path}
    }
}

impl Handler for WebsiteHandler{
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method(){
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome!!!</h2>".to_string())),
                "/dynamicView" => Response::new(StatusCode::Ok, Some("<h1>Nostalgia!!!</h2>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
            // Method::GET => todo!(),
            // Method::POST => todo!(),
            // Method::PUT => todo!(),
            // Method::DELETE => todo!(),
            // Method::PATCH => todo!(),
            // Method::OPTIONS => todo!(),
            // Method::CONNECT => todo!(),
            // Method::HEAD => todo!(),
            // Method::TRACE => todo!(),
        }
    }
}

