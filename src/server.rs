use std::net::TcpListener;

pub struct Server {
    address : String
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self){
        println!("Server listening at port: {}",self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    let a = 5;
                    println!("Ok");
                },
                Err(e) => {
                    println!("Failed to establish connection: {}", e);
                }
            }
        }
       
        
    }
}