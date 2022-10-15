use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream:connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Connected to server on 3333");
            let msg = b"hello";
            stream.write(msg).unwrap();
        }
    }
}