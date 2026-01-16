#![warn(clippy::all,
clippy::pedantic,
clippy::perf,
clippy::nursery,
// clippy::cargo,
clippy::unwrap_used,
clippy::expect_used)]

use serde::{Deserialize, Serialize};
use std::io::Write;
use std::net::TcpStream;
use tungstenite::accept;
use tungstenite::Message;

#[derive(Serialize, Deserialize, Clone)]
pub struct RustLog {
    pub log_message: String,
}

pub fn handle_client(
    tcp_addr: &str,
    stream: TcpStream,
    peer_addr: std::net::SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut websocket = match accept(stream) {
        Ok(ws) => ws,
        Err(e) => {
            println!("Error during handshake with {}: {}", peer_addr, e);
            return Err("couldn't open websocket".into());
        }
    };

    println!("WebSocket connection established with {}", peer_addr);

    // Connect TCP
    let mut tcp_stream = TcpStream::connect(tcp_addr)?;
    println!("Tcp Connected =========================================================");
    println!("Tcp Connected =========================================================");
    println!("Tcp Connected =========================================================");

    loop {
        match websocket.read() {
            Ok(Message::Text(text)) => {
                println!("Received from {}: {}", peer_addr, text);

                // Echo the message back
                let response = format!("Echo: {}", text);
                if let Err(e) = websocket.send(Message::Text(response.into())) {
                    println!("Error sending to {}: {}", peer_addr, e);
                    break;
                }
            }
            Ok(Message::Binary(data)) => {
                println!("Received {} bytes from {}", data.len(), peer_addr);

                // Echo binary data back
                if let Err(e) = tcp_stream.write_all(&data.slice(..)) {
                    println!("Error sending binary to {}: {}", peer_addr, e);
                    break;
                }
            }
            Ok(Message::Close(_)) => {
                println!("Client {} disconnected", peer_addr);
                break;
            }
            Ok(Message::Ping(data)) => {
                if let Err(e) = websocket.send(Message::Pong(data)) {
                    println!("Error sending pong to {}: {}", peer_addr, e);
                    break;
                }
            }
            Ok(Message::Pong(_)) => {
                // Handle pong if needed
            }
            Ok(Message::Frame(_)) => {
                // Handle pong if needed
            }
            Err(e) => {
                println!("Error reading from {}: {}", peer_addr, e);
                break;
            }
        }
    }

    println!("Connection with {} closed", peer_addr);
    Ok(())
}
