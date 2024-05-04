use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
    println!("Client connected: {}", stream.peer_addr().unwrap());

    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected: {}", stream.peer_addr().unwrap());
                break;
            }
            Ok(bytes_read) => {
                println!("Received {} bytes: {}", bytes_read, String::from_utf8_lossy(&buffer[..bytes_read]));
                stream.write_all(&buffer[..bytes_read]).unwrap();
            }
            Err(err) => {
                eprintln!("Error reading from socket: {}", err);
                break;
            }
        }
    }
}
