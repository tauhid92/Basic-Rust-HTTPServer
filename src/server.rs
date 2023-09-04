pub struct Server {
    address : String
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self){
       let port = &self.address[10..];
        println!("Server listening at port: {}",port);
    }
}