use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use http_core::{Method, Request};

use crate::parser::parse_request;

fn has_request_closed(req: &Request) -> bool {
    req.headers
        .get("connection")
        .is_some_and(|c| c.eq_ignore_ascii_case("close"))
}

fn handle_request(stream: &mut TcpStream, req: Request) -> bool {
    if let Ok(addr) = stream.peer_addr() {
        print!("{} :: {}\n", addr.to_string(), req);
    }

    if has_request_closed(&req) {
        return true;
    }

    let response = match req.start_line.method {
        Method::Get => "HTTP/1.1 200 OK\r\nContent-Length: 21\r\n\r\nThis is a GET request",
        Method::Post => "HTTP/1.1 200 OK\r\nContent-Length: 22\r\n\r\nThis is a POST request",
        _ => "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!",
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    false
}

fn handle_client(mut stream: TcpStream) {
    loop {
        if match parse_request(&mut stream) {
            Ok(req) => handle_request(&mut stream, req),
            Err(e) if e.kind() == std::io::ErrorKind::ConnectionAborted => true,
            Err(e) => {
                println!("Something went wrong!: {}", e);
                true
            }
        } {
            return;
        }
    }
}

pub fn run_server(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening on {} ...", addr);

    let pool = threadpool::ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| handle_client(stream)),
            Err(_) => println!("Connection failed"),
        }
    }
}
