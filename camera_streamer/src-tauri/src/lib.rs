#![warn(clippy::all,
clippy::pedantic,
clippy::perf,
clippy::nursery,
// clippy::cargo,
clippy::unwrap_used,
clippy::expect_used)]

use std::{net::TcpListener, time::Duration};

use tauri::{command, AppHandle, Emitter};

use crate::simple_stream::RustLog;

pub mod simple_stream;

#[command]
async fn start_server(app: AppHandle, tcp_addr: String) -> Result<(), String> {
    let app_clone = app.clone();
    std::thread::spawn(move || receiver_end(tcp_addr));
    tokio::time::sleep(Duration::from_millis(500)).await;
    if let Err(err) = app_clone.emit("server-running", ()) {
        println!("\n\n\n\nError on server_start: {err}\n\n\n\n");
    };
    if let Err(err) = app_clone.emit(
        "rust-log",
        RustLog {
            log_message: "[BACKEND] started server".to_string(),
        },
    ) {
        println!("\n\n\n\nError on server_start: {err}\n\n\n\n");
    };
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Testing loggin init application");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn receiver_end(tcp_addr: String) -> Result<(), std::io::Error> {
    let server = TcpListener::bind("0.0.0.0:8080")?;
    println!("WebSocket server listening on: 0.0.0.0:8080");

    for stream in server.incoming() {
        let stream = stream?;
        let peer_addr = stream.peer_addr()?;
        println!("New connection from: {}", peer_addr);

        let tcp_addr = tcp_addr.clone();

        std::thread::spawn(move || {
            let _ = simple_stream::handle_client(&tcp_addr.clone(), stream, peer_addr);
        });
    }

    Ok(())
}
