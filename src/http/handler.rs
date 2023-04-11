use super::response::Response;
use super::request::Request;
use super::status_code::StatusCode;
use super::request::ParseError;

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
        Response::new(StatusCode::OK,
                      Some("<h1>The body. It got here</h1>".to_string()))
    }
}