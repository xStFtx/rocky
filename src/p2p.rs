use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

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

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        handle_connection(stream);
                    });
                }
                Err(e) => { /* handle connection failure */ }
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    while match stream.read(&mut buffer) {
        Ok(size) => {
            // process data
            stream.write(&buffer[0..size]).unwrap();
            true
        },
        Err(_) => {
            // handle error
            false
        }
    } {}
}
