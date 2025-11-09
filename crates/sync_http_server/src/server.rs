use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use crate::parser::parse_request;

fn handle_client(mut stream: TcpStream) {
    if let Ok(addr) = stream.peer_addr() {
        print!("Connected to {}\t", addr.to_string());
    }

    match parse_request(&mut stream) {
        Ok(req) => println!("{}", req),
        Err(e) => println!("Something went wrong!: {}", e),
    }

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn run_server(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening on {} ...", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(_) => println!("Connection failed"),
        }
    }
}
