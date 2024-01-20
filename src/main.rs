use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};


fn get_response(request: Vec<String>) -> String {
    let request_line = request.iter().next().unwrap();

    let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = match fs::read_to_string(file_name) {
        Ok(c) => c,
        Err(_) => return String::from("HTTP/1.1 400 Not Found\r\n\r\n")
    };

    let length = contents.len();
    let header = vec![format!("Content-Length: {length}")];
    let header = header.join("\r\n");

    format!("{status_line}\r\n{header}\r\n\r\n{contents}\r\n")
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Request: {:#?}", http_request);
    let response = get_response(http_request);

    stream.write_all(response.as_bytes()).unwrap();
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);

        println!("Connection established!");
    }
}


