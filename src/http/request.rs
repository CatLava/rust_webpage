use super::method::Method;
pub struct Request {
    path: String,
    // We can make query string optional on requests
    query_string: Option<String>,
    method: Method,
}
