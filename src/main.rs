use server::Server;
use http::Request;
use http::Method;
// use http::request::Request;
// use http::method::Method;

mod http;
mod server;

fn main() {
    let address: String = String::from("127.0.0.1:8080".to_string());

    // let mut val: String = String::new();
    // let loc : &String = &val;
    // println!("{}", *loc);
    // execute(&mut val);
    
    let server: Server = Server::new(address);
    server.run();
}

// fn execute(val: &mut String){

// }





/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/


