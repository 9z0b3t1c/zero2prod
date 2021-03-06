//! main.rs

use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8000").expect("Failed to bind to random port");
    run(listener)?.await
}
