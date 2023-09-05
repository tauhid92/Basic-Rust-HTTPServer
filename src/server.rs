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
           let res = listener.accept();

           if res.is_err(){
            continue;
           }

           let (stream, addr) = res.unwrap();
        }
       
        
    }
}