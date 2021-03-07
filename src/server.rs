use std::net::TcpListener;

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

        let listner = TcpListener::bind(&self.addr).unwrap();

        println!("listening on: {}", self.addr);

        // Infinite loop to continually accept connections
        loop {
            // match expression requires Ok, and Err
            match listener.accept() {
                Ok((stream, _)) => {
                    println!("OK");
                },
                // _ can act as a catch all, replace err in certain cases
                Err(e) => println!("Err, failed connection : {}", e);
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
