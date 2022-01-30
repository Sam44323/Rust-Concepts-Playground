use std::io::prelude;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming gives an iterator over the connections being received in the form of a TCP-Stream
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
