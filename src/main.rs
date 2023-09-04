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
        println!("Server running at port: {}",port);
    }
}


