// Uncomment this block to pass the first stage
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    str, thread,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                //Each new incoming stream to be handled in own thread
                thread::spawn(move || handle_connection(stream));
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 256];
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("finished reading");
                break;
            }
            Ok(n) => {
                //collect the incoming request into a vec
                let incoming_cmd: Vec<_> = str::from_utf8(&buffer[..n])
                    .expect("failed to convert to string")
                    .split_whitespace()
                    .collect();

                //check if it contains ping or echo and respond accordingly
                for (idx, line) in incoming_cmd.iter().enumerate() {
                    if let true = line.to_uppercase().contains("PING") {
                        stream
                            .write_all("+PONG\r\n".as_bytes())
                            .expect("failed to write");
                    }
                    if let true = line.to_uppercase().contains("ECHO") {
                        let res = incoming_cmd[idx + 2].as_bytes();
                        stream.write_all(res).expect("failed to write echo");
                    }
                    println!("{line}");
                }
            }
            Err(e) => {
                eprintln!("failed due to this error: {e}");
            }
        }
    }
}
