use std::net::{SocketAddr, TcpStream};

use clap::Parser;

use crate::connection;

/// Connect to a remote server (client mode)
#[derive(Parser)]
pub struct Connect {
    /// The server address to connect to (e.g. 127.0.0.1:8080)
    address: SocketAddr,
}

impl Connect {
    /// Run the application in client mode (i.e. connects to the remote server)
    pub fn run(&self) -> std::io::Result<()> {
        // Connect to the address
        let stream = TcpStream::connect(&self.address)?;
        log::info!("Connected to the server: {}", &self.address);

        // Handle the connection
        connection::handle(stream)?;

        Ok(())
    }
}
