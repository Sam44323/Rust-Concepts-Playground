use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming gives an iterator over the connections being received in the form of a TCP-Stream
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection is established!");
    }
}
