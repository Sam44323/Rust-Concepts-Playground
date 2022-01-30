use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // buffer to store the data from the stream
    stream.read(&mut buffer).unwrap(); // sending the data from the stream to the buffer
    let get = b"GET / HTTP/1.1\r\n"; // the request we are looking for(converting to bytes for checking)

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // flush will send the data to the client when the entire write process is completed
}

fn main() {
    let pool = ThreadPool::new(10);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming gives an iterator over the connections being received in the form of a TCP-Stream
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // this will create a closure and will give a thread in the pool to execute the request
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
