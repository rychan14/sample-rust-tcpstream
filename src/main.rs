use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream, mut buffer: [u8; 1024], response: &[u8]) {
    stream.read(&mut buffer).expect("fails to read");
    stream.write(response).expect("fails to write");
    println!("{:?}", stream);
}

fn main() {
    let str = String::from("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\nTest");
    let resp_bytes = str.as_bytes();
    let listener = TcpListener::bind("127.0.0.1:8081").expect("fails if we can't bind");
    let buf = [0u8; 1024];
    for stream in listener.incoming() {
        match stream{
            Ok(stream) => handle_client(stream, buf, resp_bytes),
            Err(e) => println!("couldn't get client{:?}", e),
        };
    };
}
