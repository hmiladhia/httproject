use std::{
    fs,
    net::TcpStream,
    io::prelude::*,
    thread,
    time::Duration,
};

use crate::response::Response;
use crate::request::Request;

pub mod response;
pub mod request;
pub mod threadpool;


fn get_response(request: &Request) -> Response {
    let (status, file_name) = match &request.uri[..] {
        "/" => (200, "hello.html"),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            (200, "hello.html")
        }
        _ => (404, "404.html"),
    };

    let contents = match fs::read_to_string(file_name) {
        Ok(c) => c,
        Err(_) => return Response::failed(),
    };

    let length = contents.len();
    let headers = vec![format!("Content-Length: {length}")];

    Response::new(status, headers, contents)
}

pub fn handle_connection(mut stream: TcpStream) {
    let request = Request::from_stream(&mut stream);

    let response = match request {
        Ok(r) => get_response(&r),
        Err(_) => Response::empty(400),
    };

    stream.write_all(response.to_string().as_bytes()).unwrap();
}
