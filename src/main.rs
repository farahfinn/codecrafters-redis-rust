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
                println!("error: {}", e);
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
                let incoming_cmd = str::from_utf8(&buffer[..n])
                    .expect("failed to convert to string")
                    .split_whitespace();
                for line in incoming_cmd {
                    if line.contains("PING") {
                        stream
                            .write_all("+PONG\r\n".as_bytes())
                            .expect("failed to write");
                    }
                }
            }
            Err(e) => {
                eprintln!("failed due to this error: {e}");
            }
        }
    }
}
