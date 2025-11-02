use std::{io::Read, net::{TcpListener, TcpStream}};


fn stream_to_buf(mut stream: TcpStream) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;
    Ok(buf)
}

fn handle_client(stream: TcpStream) {
    if let Ok(addr) = stream.peer_addr() {
        print!("Connected to {}\t", addr.to_string());
    }

    if let Ok(buf) = stream_to_buf(stream) {
        match String::from_utf8(buf) {
            Ok(echo) => println!("{}", echo),
            Err(_) => println!("Invalid UTF-8"),
        }
    }
}

fn run_server(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening on {} ...", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(_) => println!("Connection failed"),
        }
    }
}

fn main() {
    run_server( "0.0.0.0:7878");
}
