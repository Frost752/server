use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    println!("Connected to server!");

    // Send data to the server
    let message = "Hello, server!";
    stream.write_all(message.as_bytes())?;

    println!("Sent: {}", message);

    // Receive response from the server
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    // Print the response from the server
    println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

    Ok(())
}
