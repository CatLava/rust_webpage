use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Server {
    addr: String,
}


impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {

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
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));


                            // Next part is a convert function of the incoming bytes
                            // coded in the REquest portion of this class

                            match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse request!"),
                            }

                        },
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
