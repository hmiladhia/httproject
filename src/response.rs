use std::fmt::{self, Display};

#[derive(Debug, Default)]
pub struct Response {
    status: u32,
    headers: Vec<String>,
    body: String,
}

impl Response {
    pub fn new(status: u32, headers: Vec<String>, body: String) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    pub fn empty(status: u32) -> Self {
        Self {
            status,
            ..Default::default()
        }
    }

    pub fn failed() -> Self {
        Self::empty(404)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header = self.headers.join("\r\n");

        let status_msg = match self.status {
            200 => "OK",
            400 => "BAD REQUEST",
            404 => "NOT FOUND",
            _ => "NOT FOUND",
        };

        let status = &self.status;
        let status_line = format!("HTTP/1.1 {status} {status_msg}");

        let formatted = format!(
            "{status_line}\r\n{header}\r\n\r\n{body}\r\n",
            body = self.body
        );
        write!(f, "{formatted}")
    }
}
