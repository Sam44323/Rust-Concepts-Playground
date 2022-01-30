use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // buffer to store the data from the stream

    stream.read(&mut buffer).unwrap(); // sending the data from the stream to the buffer

    println!("Request: {}", String::from_utf8_lossy(&buffer[..])); // from_utf8_lossy is a method that converts a slice of bytes into string including invalid character
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming gives an iterator over the connections being received in the form of a TCP-Stream
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
