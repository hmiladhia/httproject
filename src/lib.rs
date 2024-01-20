use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use crate::response::Response;


pub mod response;


fn get_response(request: Vec<String>) -> Response {
    let request_line = request.iter().next().unwrap();

    let uri = request_line.split(" ").skip(1).next().unwrap();

    let (status, file_name) = match uri {
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
    // 
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = get_response(http_request);

    stream.write_all(response.to_string().as_bytes()).unwrap();
}
