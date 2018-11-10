use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::path::Path;

const STATIC_PATH: &str = "static";
const WELCOME_HTML: &str = "welcome.html";
const NOT_FOUND_HTML: &str = "404.html";
const GET: &str = "GET / HTTP/1.1\r\n";
const STATUS_OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
const STATUS_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8008").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("connection established!: {:?}", stream);
        handle(stream);
    }
}

fn handle(mut stream: TcpStream) {
    let mut buf = [0u8; 512];
    stream.read(&mut buf).unwrap();
    let message = String::from_utf8_lossy(&buf[..]);
    println!("message:\n\n{}", message);

    if message.starts_with(GET) {
        deliver_page(STATUS_OK, WELCOME_HTML, &mut stream);
    }

    else {
        deliver_page(STATUS_NOT_FOUND, NOT_FOUND_HTML, &mut stream)
    }
}

fn append_html(http_response: &str, html_path: &str) -> String {
    format!("{}{}", http_response, fs::read_to_string(Path::new(STATIC_PATH).join(html_path)).unwrap())
}

fn deliver_page(status_line: &str, page_name: &str, stream: &mut TcpStream) {
    let response = append_html(status_line, page_name);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("response sent successfully");
}




