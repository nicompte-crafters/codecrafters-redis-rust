// Uncomment this block to pass the first stage
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

fn main() {
    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_stream(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    stream.write_all(b"$4\r\nPONG\r\n").unwrap();
}
