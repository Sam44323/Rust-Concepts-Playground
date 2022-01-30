use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // buffer to store the data from the stream

    stream.read(&mut buffer).unwrap(); // sending the data from the stream to the buffer

    let content = fs::read_to_string("index.html").unwrap(); // reading the file

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // flush will send the data to the client when the entire write process is completed
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming gives an iterator over the connections being received in the form of a TCP-Stream
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
