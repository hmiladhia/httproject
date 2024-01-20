use std::{fs, io::prelude::*, net::TcpStream, path::Path};

use crate::request::Request;
use crate::response::Response;

pub mod request;
pub mod response;
pub mod threadpool;


fn get_response(request: &Request, base_path: &Path) -> Response {
    let file_name = match &request.uri[..] {
        "/" => "/index",
        uri => uri,
    };

    let file_name = if file_name.starts_with("/") {
        &file_name[1..]
    } else {
        file_name
    };

    let p = base_path.join(&file_name).with_extension("html");

    let (status, p) = match p.exists() {
        true => (200, p),
        false => (404, base_path.join("errors").join("404.html"))
    };

    let contents = match fs::read_to_string(p) {
        Ok(c) => c,
        Err(_) => return Response::failed(),
    };

    let length = contents.len();
    let headers = vec![format!("Content-Length: {length}")];

    Response::new(status, headers, contents)
}

pub fn handle_connection(mut stream: TcpStream, base_path: &Path) {
    let request = Request::from_stream(&mut stream);

    let response = match request {
        Ok(r) => get_response(&r, base_path),
        Err(_) => Response::empty(400),
    };

    stream.write_all(response.to_string().as_bytes()).unwrap();
}
