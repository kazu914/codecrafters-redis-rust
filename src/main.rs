// Uncomment this block to pass the first stage
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    process,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    println!("Started Redis Server");
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => handle_connection(_stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let response = "+PONG\r\n";
    let mut buf = [0; 512];
    loop {
        stream.read(&mut buf).unwrap_or_else(|_| process::exit(1));
        stream.write_all(response.as_bytes()).unwrap_or_else(|_| process::exit(1));
    }
}
