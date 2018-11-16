extern crate web_server;

use web_server::ThreadPool;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

const WELCOME_HTML: &str = include_str!("static/welcome.html");
const NOT_FOUND_HTML: &str = include_str!("static/404.html");
const GET_INDEX: &str = "GET / HTTP/1.1\r\n";
const GET_SLEEP: &str = "GET /sleep HTTP/1.1\r\n";
const STATUS_OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
const STATUS_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";


fn main() {
    serve_forever("127.0.0.1", 8008);
}

fn serve_forever(host:&str, port: u16) {
    let af_inet = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&af_inet).unwrap();
    println!("serving forever on: http://{}\n...", &af_inet);
    let pool = ThreadPool::new(30);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("connection established!: {:?}", stream);
        pool.execute(||{handle(stream)});
    }

}

fn handle(mut stream: TcpStream) {
    let mut buf = [0u8; 512];
    stream.read(&mut buf).unwrap();
    let message = String::from_utf8_lossy(&buf[..]);
    println!("message:\n\n{}", message);

    if message.starts_with(GET_INDEX) {
        deliver_page(STATUS_OK, WELCOME_HTML, &mut stream);
    }
    else if message.starts_with(GET_SLEEP) {
        thread::sleep(Duration::from_secs(5));
        deliver_page(STATUS_OK, WELCOME_HTML, &mut stream)
    }

    else {
        deliver_page(STATUS_NOT_FOUND, NOT_FOUND_HTML, &mut stream)
    }
}

fn append_html(http_response: &str, html: &str) -> String {
    format!("{}{}", http_response, html)
}

fn deliver_page(status_line: &str, page_name: &str, stream: &mut TcpStream) {
    let response = append_html(status_line, page_name);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("response sent successfully");
}




