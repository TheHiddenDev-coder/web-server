#[allow(unused)]
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

pub fn get_main_page(mut stream: TcpStream) {
    let contents = fs::read_to_string("index.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    let _ = stream.flush().unwrap();
}

pub fn get_404(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = fs::read_to_string("404.html").unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
