use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/// cargo r --bin s-web-server
///
/// ### 请求格式:
///```
/// Method Request-URI HTTP-Version CRLF
/// headers CRLF
/// message-body
/// ```
/// ### 响应格式:
/// ```
/// HTTP-Version Status-Code Reason-Phrase CRLF
/// headers CRLF
/// message-body
/// ```
/// CRLF 序列也可以写成 \r\n
///
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
