pub mod structures;
pub mod managers;
pub mod plugins;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

use crate::plugins::plugin_docs::PluginDocs;
use crate::plugins::plugin_static::PluginStatic;
use crate::structures::{SmnRequest, SmnResponse};
use crate::managers::PluginManager;

fn handle_client(mut stream: TcpStream, plugin_manager: &PluginManager) {
    let mut buffer = Vec::new();
    let mut temp = [0u8; 512];

    let header_end;

    loop {
        match stream.read(&mut temp) {
            Ok(0) => return, // connection closed
            Ok(n) => {
                buffer.extend_from_slice(&temp[..n]);

                if let Some(pos) = buffer
                    .windows(4)
                    .position(|w| w == b"\r\n\r\n")
                {
                    header_end = pos + 4;
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                return;
            }
        }
    }

    // 🔴 CRITICAL FIX: only parse the first request
    let request_bytes = &buffer[..header_end];

    let request = match SmnRequest::from_buffer(request_bytes) {
        Ok(req) => {
            req
        }
        Err(e) => {
            eprintln!("Failed to parse request: {}", e);

            let response = SmnResponse::new(
                400,
                "Bad Request",
                b"Bad Request".to_vec(),
            )
            .with_header("Content-Type", "text/plain");

            let _ = stream.write_all(&response.to_bytes());
            return;
        }
    };

    let response = plugin_manager.route(&request);

    let _ = stream.write_all(&response.to_bytes());
}


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25333")?;
    println!("Server listening on http://127.0.0.1:25333");

    // ---- Plugin system bootstrap ----
    let mut plugin_manager = PluginManager::new();
    plugin_manager.register(Box::new(PluginDocs::new()));
    plugin_manager.register(Box::new(PluginStatic { root: PathBuf::from("res/static") }));
    plugin_manager.init_all();
    // ---------------------------------

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &plugin_manager);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
