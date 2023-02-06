use std::net::TcpListener;
use std::net::TcpStream;

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
            listener.accept();
        }
    }
}
