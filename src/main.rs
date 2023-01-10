// Uncomment this block to pass the first stage
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    println!("Started Redis Server");
    loop {
        let (socket, _) = listener.accept().unwrap();
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let response = "+PONG\r\n";
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            break;
        }
        stream.write_all(response.as_bytes()).unwrap();
    }
}
