use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::path::Path;

const STATIC_PATH: &str = "static";
const WELCOME_HTML: &str = "welcome.html";
const GET: &str = "GET / HTTP/1.1\r\n";

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
        get_page(WELCOME_HTML, &mut stream);
    }
}

fn append_html(http_response: &str, html_path: &str) -> String {
    format!("{}{}", http_response, fs::read_to_string(Path::new(STATIC_PATH).join(html_path)).unwrap())
}

fn get_page(page_name: &str, stream: &mut TcpStream) {
    let response = append_html("HTTP/1.1 200 OK\r\n\r\n", page_name);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("response sent successfully");
}


