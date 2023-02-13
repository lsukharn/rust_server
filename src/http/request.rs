use super::method::Method;
use std::convert::TryFrom;
// use std::fmt::Error;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Debug, Formatter};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method // GET, POST etc
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(value: &[u8]) -> Result<Self, Self::Error>{
        unimplemented!()
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

impl Error for ParseError {

}

