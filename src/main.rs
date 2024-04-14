use std::net::{TcpStream, TcpListener};
use std::io::{Write};
use anyhow::Error;

fn handle_client(stream: &mut TcpStream) -> Result<(), Error> {
    stream.write_all("+PONG\r\n".as_bytes())?;
    Ok(())
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
         match stream {
             Ok(mut stream) => {
                 handle_client(&mut stream);
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
    }
}
