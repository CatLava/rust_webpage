use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs;

pub struct WebsiteHandler {
    // defining public paths for internet access
    public_path : String
}

//
impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        // this is a check for directory traversal
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal attempt: {}", file_path );
                    None
                }
            }
            Err(_) => None,
        }
        // need to convert this result into an option
        // Use Ok option in this case to convert
        //fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response{
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string())),
                // implementation for style.css, bad security vulnerability as well
                // This opens up to directory traversal for the website
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            }
            _ => Response::new(StatusCode::NotFound, None),
        }


        // Prior implementation below
        // Response::new(StatusCode::Ok, Some("<h1>Test</h1>".to_string()))

    }
}
