#![allow(dead_code)]
use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let address: String = String::from("127.0.0.1:8080".to_string());

    let server: Server = Server::new(address);
    server.run(WebsiteHandler);
}
