use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use std::fmt::{Display, self};


#[derive(Debug, Default)]
struct Response {
    status: u32,
    headers: Vec<String>,
    body: String,
}

impl Response {
    fn new(status: u32, headers: Vec<String>, body: String) -> Self {
        Self {
            status,
            headers,
            body
        }
    }

    fn empty(status: u32) -> Self{
        Self {
            status,
            ..Default::default()
        }
    }

    fn failed() -> Self {
        Self::empty(404)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header = self.headers.join("\r\n");

        let status_msg = match self.status {
            200 => "OK",
            404 => "NOT FOUND",
            _ => "NOT FOUND",
        };

        let status = &self.status;
        let status_line = format!("HTTP/1.1 {status} {status_msg}");

        let formatted = format!("{status_line}\r\n{header}\r\n\r\n{body}\r\n", body=self.body);
        write!(f, "{formatted}")
    }

}



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

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = get_response(http_request);

    stream.write_all(response.to_string().as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);

        println!("Connection established!");
    }
}
