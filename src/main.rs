// Uncomment this block to pass the first stage
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
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
                thread::spawn(move || handle_connection(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 256];

    while let Ok(n) = stream.read(&mut buffer) {
        let response = "+PONG\r\n".as_bytes();
        println!("{:?}", &buffer[..n].make_ascii_uppercase());
        stream.write_all(response).expect("failed to write");
    }
}
