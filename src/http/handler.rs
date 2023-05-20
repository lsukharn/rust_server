use super::response::Response;
use super::request::Request;
use super::status_code::StatusCode;
use super::request::ParseError;
use super::method::Method;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {  // default implementation
        println!("Failed to parse the request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct WebHandler;

impl Handler for WebHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, Some("<h1>Welcome</h1>".to_string())),
                "/hello" => Response::new(StatusCode::OK,Some("<h1>Hello!</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None)
            },
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}