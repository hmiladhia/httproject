use std::{io::{prelude::*, BufReader}, net::TcpStream};

#[derive(Debug, Default)]
pub struct Request {
    pub uri: String,
}

impl Request {
    pub fn new(uri: String) -> Self {
        Self { uri }
    }

    pub fn from_stream(stream: &mut TcpStream) -> Result<Self, String> {
        let buf_reader = BufReader::new(stream);
        let mut http_request = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty());

        let request_line = http_request.next().unwrap();
        let uri = match request_line.split(" ").skip(1).next() {
            Some(uri) => uri,
            None => return Err(String::from("Could Not parse Request")),
        };

        Ok(Request::new(String::from(uri)))
    }
}
