// Uncomment this block to pass the first stage
use std::{
    char,
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
    loop {
        let mut buf = [0; 512];
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            break;
        }

        let data = String::from_utf8_lossy(&buf);
        let trimmed_data = data.trim_end_matches(char::from(0));
        let data_array = trimmed_data.split("\r\n").collect::<Vec<&str>>();
        match data_array[2] {
            "ECHO" | "echo" => {
                let response = "+".to_owned() + data_array[4] + "\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
            _ => {
                let response = "+PONG\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
        };
    }
}
