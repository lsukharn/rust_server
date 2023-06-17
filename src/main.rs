#![allow(dead_code)]
use server::Server;
use http::handler::WebHandler;
use std::env;

mod server;
mod http;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let server = Server::new("127.0.0.1:8080".to_string()); // struct
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    server.run(WebHandler::new(public_path));
}
