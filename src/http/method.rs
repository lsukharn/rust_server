#[repr(u8)]
pub enum Method {
    GET(String),
    POST,
    PUT = 5,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    DELETE(u64)
}