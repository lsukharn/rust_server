use std::fmt::{Result as FmtResult, Display, Debug, Formatter};
use std::string::ParseError;
use std::net::TcpStream;
use std::io::{Result as IoResult, Write};
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {status_code, body}
    }

    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        let body = match &self.body {
                Some(s) => s,
                None => "None"
            };
        write!(stream, "HTTP/1.1 {} {} \r\n\r\n {}", self.status_code, self.status_code.reason_phrase(), body)
    }
}