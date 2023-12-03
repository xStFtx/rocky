use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::thread;
use std::sync::Arc;
use log::{info, error};

pub struct P2P {
    port: u16,
}

impl P2P {
    pub fn new(port: u16) -> Self {
        P2P { port }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .expect("Failed to bind to port");
        let listener = Arc::new(listener);

        info!("P2P server started on port {}", self.port);

        for stream in listener.incoming() {
            let listener = Arc::clone(&listener);
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        handle_connection(stream).unwrap_or_else(|e| error!("Failed to handle connection: {}", e));
                    });
                }
                Err(e) => {
                    error!("Connection failed: {}", e);
                }
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let size = stream.read(&mut buffer)?;
        if size == 0 { break; } // Connection was closed

        // Process the data...
        // For example, this could involve parsing the data according to a protocol

        stream.write_all(&buffer[0..size])?;
    }

    info!("Connection with {} closed.", stream.peer_addr()?);
    Ok(())
}
