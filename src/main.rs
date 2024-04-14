use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read};
use anyhow::Error;

const PING_COMMAND: [u8; 14] = [0x2a, 0x31, 0xd, 0xa, 0x24, 0x34, 0xd, 0xa, 0x70, 0x69, 0x6e, 0x67, 0xd, 0xa];

fn handle_client(stream: &mut TcpStream) -> Result<(), Error> {
    loop {
        let mut buf = [0u8; 128];
        let n = stream.read(&mut buf)?;

        if !buf[..n].starts_with(&PING_COMMAND) {
            return Ok(());
        }
        stream.write_all("+PONG\r\n".as_bytes())?;
    }
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
         match stream {
             Ok(mut stream) => {
                 match handle_client(&mut stream) {
                     Ok(_) => (),
                     Err(error) => eprintln!("Error handling connection: {}", error),
                 }
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
    }
}
