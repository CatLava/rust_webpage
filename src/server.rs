use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;

// handle incoming requests
pub trait Handler{
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request, {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}


pub struct Server {
    addr: String,
}


impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

// impllementing handler into run function
    pub fn run(self, mut handler: impl Handler) {

        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("listening on: {}", self.addr);

        // Infinite loop to continually accept connections
        loop {
            // match expression requires Ok, and Err
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("OK");
                    // setting buffer limit for incoming data streams
                    let mut buffer = [0; 1024];
                    // Need to include Read as trait in library include
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));


                            // Next part is a convert function of the incoming bytes
                            // coded in the REquest portion of this class
                            // making match statement a variable to match on
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                    // write!(stream, "{}", response);
                                    // removed above bc implemented in request
                                },
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }

                            };
                            if let Err(e) = response.send(&mut stream){
                                println!("Failed to send response: {}", e);

                            }

                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                // _ can act as a catch all, replace err in certain cases
                Err(e) => println!("Err, failed connection : {}", e),
            }

        }

    }
}




/*
If we didn't use match expression to validate connections
This would be a way to fo this with if statement, etc, etc
            let res = listener.accept();

            // If unvalid connection, continue
            if res.is_err() {
                continue;
            }
            // Unwrap provides to elements we are placing in a tuble
            let (stream, addr) = res.unwrap();
*/
