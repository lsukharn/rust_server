use super::method::Method;
use std::str;
use std::str::Utf8Error;
use std::convert::TryFrom;
// use std::fmt::Error;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Debug, Formatter};
use crate::http::method::MethodError;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method // GET, POST etc
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error>{
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = method.parse()?;

        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)>{
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r'{
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

impl From<Utf8Error> for ParseError {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(value: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding, // not utf8 encoded
    InvalidProtocol, // diffrenet than HTTP1.1 protocol
    InvalidMethod
}

impl ParseError {
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}
