#![allow(dead_code)]
use server::Server;
use http::Request;
use http::Method;
use http::handler::WebHandler;
use std::io::{Read, Error as IOError};
use std::fs::File;

use std::str;

mod server;
mod http;

fn main() {
    // let string = String::from("127.0.0.1:8080");
    // let string_slice = &string[10..]; //give me everything after the 10yth byte, NOT 10th character!
    // let string_borrow: &str = &string;
    // let string_literal = "1234"; // this srtring slice is immutable
    //
    // dbg!(&string);
    // dbg!(string_slice);
    // dbg!(string_borrow);
    // dbg!(string_literal);

    // let get = Method::GET("abcd".to_string());
    // let delete = Method::DELETE(100);
    // let post = Method::POST;
    // let put = Method::PUT;
    let server = Server::new("127.0.0.1:8080".to_string()); // struct
    server.run(WebHandler);
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
 */