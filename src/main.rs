use server::Server;
use http::Request;
use http::Method;
// use http::request::Request;
// use http::method::Method;

mod http;
mod server;

fn main() {
    let address: String = String::from("127.0.0.1:8080".to_string());

    let server: Server = Server::new(address);
    server.run();
}



