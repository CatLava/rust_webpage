#![allow(dead_code)]

use server::Server;
use http::Request;
use http::Method;
use std::net::TcpListener;
// pulling below in for environment variables
use std::env;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // environment variable for path
    // created public directory for public path
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path : {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}

// Everything inside of moduleis private by default.
// need pub keyword to use outside module
