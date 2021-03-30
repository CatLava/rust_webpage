use std::fmt::{Display, Formatter, Result as FmtResult};

// mapping an enum to associated values
// implementing clone and copy trait for Rust
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self{
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        // need to cast the enum as integer for response code
        // unsigned int
        write!(f, "{}", *self as u16)
    }
}
