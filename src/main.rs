use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use c0de::core::threadpool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:52013").unwrap();
    let pool = ThreadPool::new(10);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.exec(move || {
            handle_connect(stream);
        });
    }
}

fn handle_connect(mut stream: TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();
    let (status_line, content) = if buf.starts_with(b"GET / HTTP/1.1") || buf.starts_with(b"GET /?") || buf.starts_with(b"GET /#") {
        let content = std::fs::read_to_string("hello.html").unwrap();
        (format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\nContent-Type: text/html; charset=UTF-8\r\nConnection: keep-alive\r\n\r\n", content.len()), content)
    } else {
        let content = std::fs::read_to_string("404.html").unwrap();
        (format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length:{}\r\nContent-Type: text/html; charset=UTF-8\r\nConnection: keep-alive\r\n\r\n", content.len()), content)
    };

    let response = format!("{}{}", status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
