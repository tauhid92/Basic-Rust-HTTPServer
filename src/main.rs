#![allow(dead_code)]
use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod http;
mod server;
mod website_handler;

fn main() {
    
    let public_path = env::var("PUBLIC_PATH").unwrap();
    let server: Server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
