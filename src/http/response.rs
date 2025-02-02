use super::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

pub struct Response{
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body}
    }
    // making body not allotcated to heap
    // body responses can be huge in high perforamnce web pages
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
             stream,
             "HTTP/1.1 {} {}\r\n\r\n{}",
             self.status_code,
             self.status_code.reason_phrase(),
             body,
         )

    }
}

/*
// implementing display trait for Response
impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
             "HTTP/1.1 {} {}\r\n\r\n{}",
             self.status_code,
             self.status_code.reason_phrase(),
             body

         )
    }
}
*/
