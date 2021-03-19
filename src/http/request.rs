use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str;
use std::str::Utf8Error;


// we are adding lifetimes to this struct, the pub struct
// this case the buffer is assigned a lifetime value '
pub struct Request<'buf> {
    path: &'buf str,
    // We can make query string optional on requests
    query_string: Option<&'buf str>,
    method: Method,
}


// various lifetimes added as part of this code
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP 1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
         // Using or as a method from utf8,  there will still be a Result
         // line below is error handling, rather than  a match
         // implemented a question mark for handling of errors here
        let request = str::from_utf8(buf)?;

        // let new a split, creating a new variable
        // old variable is unusable
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        // splitting the 2nd half of the parameter
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        // Matching in querystring looking for question mark
        // There is an if let to help quantify this
        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

//function to split up requests
// as request comes in, need to update this
fn get_next_word(request: &str) ->  Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        // If a space encountered
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]));
        }
    }

    None
}


// creating enum for error handling upon these requests
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError{
    fn message(&self) -> &str{
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}
// Method Error if one is Received
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self{
        Self::InvalidMethod
    }
}


impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncoding
    }
}
// required for error trait
impl Display for ParseError {
    // this was grabbed from ParseError code
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

//required for error trait
impl Debug for ParseError {
    // this was grabbed from ParseError code
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// Use trait from standard library to make more idiomatic
// What are Rust traits ?
impl Error for ParseError {

}
