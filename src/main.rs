// Uncomment this block to pass the first stage
use std::{
    io::prelude::*,
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            println!("client closed the connection");
            break;
        }
        stream.write("+PONG\r\n".as_bytes()).unwrap();
    }
}
