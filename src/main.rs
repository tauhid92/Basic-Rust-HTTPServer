fn main() {
    let address: String = String::from("127.0.0.1.8080");

    let server: Server = Server::new(address);
    server.run();
}
struct Server {
    address : String
}

impl Server {
    fn new(address: String) -> Self {
        Self { address }
    }

    fn run(self){
       let port = &self.address[10..];
        println!("Server listening at port: {}",port);
    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

enum Method{
    GET, POST, PUT, DELETE, PATCH, OPTIONS, CONNECT, HEAD, TRACE
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/


