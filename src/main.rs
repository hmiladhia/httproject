use std::{
    net::TcpListener,
    thread,
};

use httproject::handle_connection;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);

        println!("Connection established!");
    }
}
