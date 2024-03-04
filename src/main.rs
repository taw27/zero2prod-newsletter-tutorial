use std::{io, net::TcpListener};

use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to bind address");

    run(listener)?.await
}
