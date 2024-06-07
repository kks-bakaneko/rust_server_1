use std::net::TcpListener;
use std::io::{Read, Write};
use std::str;

fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");
    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connectin established");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Got response from server:{:?}", str::from_utf8(&buffer).unwrap());
        stream.write(&mut buffer).unwrap();
    }
}
