use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

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

    stream.read(&mut buf);
    println!("message: {}", String::from_utf8_lossy(&buf[..]));
}
