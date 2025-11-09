mod parser;
mod server;

fn main() {
    server::run_server("0.0.0.0:7878");
}
