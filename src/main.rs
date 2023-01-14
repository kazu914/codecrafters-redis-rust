// Uncomment this block to pass the first stage
use anyhow::Result;
use resp::RespConnection;
use resp::Value::{Error, SimpleString};
use tokio::net::{TcpListener, TcpStream};
mod resp;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    println!("Started Redis Server");
    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    handle_connection(stream).await.unwrap();
                });
            }
            Err(e) => {
                println!("ERROR: {}", e);
            }
        }
    }
}

async fn handle_connection(stream: TcpStream) -> Result<()> {
    let mut conn = RespConnection::new(stream);

    loop {
        let value = conn.read_value().await?;

        if let Some(value) = value {
            let (command, args) = value.to_command()?;
            let response = match command.to_ascii_lowercase().as_ref() {
                "ping" => SimpleString("PONG".to_string()),
                "echo" => args.first().unwrap().clone(),
                _ => Error(format!("command not implemented: {}", command)),
            };

            conn.write_value(response).await?;
        } else {
            break;
        }
    }

    Ok(())
}
