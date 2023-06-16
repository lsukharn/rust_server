use super::response::Response;
use super::request::Request;
use super::status_code::StatusCode;
use super::request::ParseError;
use super::method::Method;
use std::fs;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {  // default implementation
        println!("Failed to parse the request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct WebHandler{
    public_path: String
}

impl WebHandler {
    pub fn new(public_path: String) -> Self {
        Self{ public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String>{
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                }
                else {
                    println!("Directory Traversal Attempted!, {}", file_path);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for WebHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::OK,self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::OK, Some(contents)),
                    _ =>  Response::new(StatusCode::NotFound, None)
                }
            },
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}