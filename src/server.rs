use std::io::Read;
use crate::http::Request;
use crate::http::handler::Handler;
use std::convert::TryFrom;
use std::net::TcpListener;


pub struct Server {
    // struct definition
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self { // Self is an alias for struct name! Self and Server are interchangeble
        Self {
            addr
        }
    }
    pub fn run(self, mut handler: impl Handler) {  // take ownership of the struct
        println!("Listerning on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, adrr)) =>{
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            let response= match Request::try_from(&buf[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e)
                            };
                            response.send(&mut stream);
                        },
                        Err(e) => println!("Failed to read form connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
        }
    }
}
