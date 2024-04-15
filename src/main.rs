use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read};
use anyhow::Error;
use std::thread;


#[derive(PartialEq)]
enum CommandType {
    PING,
    ECHO,
}

struct Command {
    c_type: CommandType,
    args: Vec<String>,
}

impl Command {
    fn new() -> Command {
        Command {
            c_type: CommandType::PING,
            args: Vec::<String>::new(),
        }
    }
}

fn parse_command(buf: &mut [u8]) -> Command {
    let mut c = Command::new();
    let mut buf_iter = buf.iter();

    let mut has_command_type = false; // if parser already knows the command and is onto the args
    let mut in_string = false;
    let mut array_count = usize::MAX; // count of how many array elements are left


    while let Some(&byte) = buf_iter.next() {
        // println!("{:}, {:}", in_string, array_count);
        if in_string {
            let string_bytes = buf_iter.by_ref().take_while(|b| **b != u8::from(0xd)).cloned().collect::<Vec<u8>>();
            let string = std::str::from_utf8(&string_bytes).expect("Invalid String (error at converting from utf8)");
            // println!("string: {:?}", string.as_bytes());
            if has_command_type {
                c.args.push(String::from(string));
            } else {
                match string {
                    "echo" => c.c_type = CommandType::ECHO,
                    "ping" => c.c_type = CommandType::PING,
                    &_ => todo!("Command Not Implemented!"),
                }
                has_command_type = true;
            }
            array_count -= 1;
            in_string = false;
        }
        if byte == u8::from(0x2a) { // start of an array
            let digit_bytes = buf_iter.by_ref().take_while(|b| **b != u8::from(0xd)).cloned().collect::<Vec<u8>>();
            let digits = std::str::from_utf8(&digit_bytes);
            if let Ok(num_args) = digits.expect("Invalid Command").parse::<usize>() {
                // println!("num_args {}", num_args);
                array_count = num_args.clone();
            }
        } else if byte == u8::from(0x24) { // start of a string
            let digit_bytes = buf_iter.by_ref().take_while(|b| **b != u8::from(0xd)).cloned().collect::<Vec<u8>>();
            let digits = std::str::from_utf8(&digit_bytes);
            if let Ok(s_len) = digits.expect("Invalid Command").parse::<usize>() {
                // println!("string_length {}", s_len);
                in_string = true;
            }
        }
        if array_count == 0 { return c; }
    }

    c
}

fn handle_client(stream: &mut TcpStream) -> Result<(), Error> {
    loop {
        let mut buf = [0u8; 128];
        let n = stream.read(&mut buf)?;
        if n == 0 { return Ok(()); }
        let command = parse_command(&mut buf[..n]);

        if command.c_type == CommandType::PING {
            stream.write_all("+PONG\r\n".as_bytes())?;
        } else if command.c_type == CommandType::ECHO {
            stream.write_all(format!("+{}\r\n", command.args.get(0).expect("there is nothing to echo")).as_bytes())?;
        } else {
            println!("bytes {:?}", &buf[..n]);
            return Ok(());
        }
    }
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || match handle_client(&mut stream) {
                    Ok(_) => (),
                    Err(error) => println!("Error handling connection: {}", error),
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
