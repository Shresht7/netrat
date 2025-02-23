use std::net::{TcpListener, TcpStream};

use clap::Args;

use netrat::{connection, helpers::Address};

/// Create a relay server
#[derive(Args)]
pub struct Relay {
    /// The target server to connect to
    target: Address,
    /// The local address to listen for a client connection
    client: Address,
}

impl Relay {
    pub fn run(&self) -> std::io::Result<()> {
        // Connect to the target server
        let target_stream = TcpStream::connect(&self.target)?;
        log::info!("Connected to target server: {}", self.target);

        // Bind to the client address and accept one connection
        let listener = TcpListener::bind(&self.client)?;
        let (client_stream, client_addr) = listener.accept()?;
        log::info!("Accepted client connection from {}", client_addr);

        // Handle relay connection
        connection::relay::handle(client_stream, target_stream)
    }
}
