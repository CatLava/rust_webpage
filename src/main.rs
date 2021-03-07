use server::Server;
use http::Request;
use http::Method;
use std::net::TcpListener;

mod server;
mod http;

fn main() {



    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

// Everything inside of moduleis private by default.
// need pub keyword to use outside module
