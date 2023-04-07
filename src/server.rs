use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::TcpListener;

pub struct Server {
    // struct definition
    addr: String,
}

impl Server {
    // functionality
    // erither a method of an associated fucntion
    // "methods" are similar to methods in OOP langs
    // "associated function" associated with struct type,
    // but they don't need instance of the struct.Like static functions in OOP langs
    pub fn new(addr: String) -> Self { // Self is an alias for struct name! Self and Server are interchangeble
        Self {
            addr
        }
    }
    pub fn run(self) {  // take ownership of the struct
        println!("Listerning on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, adrr)) =>{
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                },
                                Err(e) => println!("Failed to parse the request: {}", e)
                            }
                        },
                        Err(e) => println!("Failed to read form connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
        }
    }
}
