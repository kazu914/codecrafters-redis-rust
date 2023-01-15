use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

// Uncomment this block to pass the first stage
use anyhow::Result;
use bytes::Bytes;
use resp::RespConnection;
use resp::Value::{Error, Null, SimpleString};
use tokio::net::{TcpListener, TcpStream};
mod resp;

struct Item {
    value: Bytes,
    expired_at: Option<SystemTime>,
}

type Db = Arc<Mutex<HashMap<String, Item>>>;
#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    println!("Started Redis Server");

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let db = db.clone();
        let incoming = listener.accept().await;
        match incoming {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    handle_connection(stream, db).await.unwrap();
                });
            }
            Err(e) => {
                println!("ERROR: {}", e);
            }
        }
    }
}

async fn handle_connection(stream: TcpStream, db: Db) -> Result<()> {
    let mut conn = RespConnection::new(stream);

    loop {
        let value = conn.read_value().await?;

        if let Some(value) = value {
            let (command, args) = value.to_command()?;
            let response = match command.to_ascii_lowercase().as_ref() {
                "ping" => SimpleString("PONG".to_string()),
                "echo" => args.first().unwrap().clone(),
                "set" => {
                    let mut db = db.lock().unwrap();
                    let key = args[0].unwrap_bulk();
                    let balue_bytes = Bytes::from(args[1].unwrap_bulk());
                    if args.len() > 3 {
                        // TODO: implement other than mills
                        let ttl = args[3].unwrap_bulk().parse().unwrap();
                        let expired_at = SystemTime::now().add(Duration::from_millis(ttl));
                        let item = Item {
                            value: balue_bytes,
                            expired_at: Some(expired_at),
                        };
                        db.insert(key, item);
                    } else {
                        let item = Item {
                            value: balue_bytes,
                            expired_at: None,
                        };
                        db.insert(key, item);
                    }
                    SimpleString("OK".to_string())
                }
                "get" => {
                    let db = db.lock().unwrap();
                    let item = db.get(&args[0].unwrap_bulk()).unwrap();
                    if let Some(expired_at) = item.expired_at {
                        if SystemTime::now().gt(&expired_at) {
                            Null
                        } else {
                            SimpleString(String::from_utf8(item.value.to_vec()).unwrap())
                        }
                    } else {
                        SimpleString(String::from_utf8(item.value.to_vec()).unwrap())
                    }
                }
                _ => Error(format!("command not implemented: {}", command)),
            };

            conn.write_value(response).await?;
        } else {
            break;
        }
    }

    Ok(())
}
