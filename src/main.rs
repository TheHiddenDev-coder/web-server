use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
mod get_req_handler;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_index = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get_index) {
        get_req_handler::get_main_page(stream);
    } else {
        get_req_handler::get_404(stream);
    }
}
