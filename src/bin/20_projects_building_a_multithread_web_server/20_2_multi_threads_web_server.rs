mod thread_pool;

use std::{fs, thread};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use crate::thread_pool::ThreadPool;

/// cargo r --bin 20_2
///
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.expect("connection wasn't established...");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // 只需要读取第一行，判断具体的 HTTP METHOD即可
    let request_line = match buf_reader.lines().next() {
        Some(next) => match next {
            Ok(req) => req,
            Err(_) => String::from("GET /error HTTP/1.1"),
        },
        None => String::from("GET /error HTTP/1.1"),
    };

    let (response_line, content_file) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            thread::sleep(Duration::from_secs(1));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => {
            thread::sleep(Duration::from_secs(7));
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    };

    let contents= match fs::read_to_string(content_file) {
        Ok(contents) => contents,
        Err(_) => format!("Can't read file: {content_file}"),
    };
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}", response_line, contents.len(), contents
    );

    if let Err(_) = stream.write_all(response.as_bytes()) {
        println!("Failed to response");
    }
}