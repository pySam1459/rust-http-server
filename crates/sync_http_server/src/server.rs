use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use http_core::{Method, Request};

use crate::parser::parse_request;

fn handle_request(mut stream: TcpStream, req: Request) {
    let response = match req.start_line.method {
        Method::Get => "HTTP/1.1 200 OK\r\nContent-Length: 21\r\n\r\nThis is a GET request",
        Method::Post => "HTTP/1.1 200 OK\r\nContent-Length: 22\r\n\r\nThis is a POST request",
        _ => "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!",
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_client(mut stream: TcpStream) {
    if let Ok(addr) = stream.peer_addr() {
        print!("Connected to {}\t", addr.to_string());
    }

    match parse_request(&mut stream) {
        Ok(req) => handle_request(stream, req),
        Err(e) => println!("Something went wrong!: {}", e),
    }
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
