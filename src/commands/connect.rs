use std::net::{TcpStream, UdpSocket};

use clap::Parser;

use netrat::helpers::{Address, Protocol};

use crate::connection;

/// Connect to a remote server (client mode)
#[derive(Parser)]
pub struct Connect {
    /// The server address to connect to (e.g. 127.0.0.1:8080)
    address: Address,

    /// The protocol to use
    #[arg(short, long, alias = "mode", default_value = "tcp")]
    protocol: Protocol,
}

impl Connect {
    /// Run the application in client mode (i.e. connects to the remote server)
    pub fn run(&self) -> std::io::Result<()> {
        match self.protocol {
            Protocol::TCP => self.run_tcp(),
            Protocol::UDP => self.run_udp(),
        }
    }

    fn run_tcp(&self) -> std::io::Result<()> {
        // Connect to the address
        let stream = TcpStream::connect(&self.address)?;
        log::info!("Connected to the server: {}", &self.address);

        // Handle the connection
        connection::tcp::handle(stream)?;

        Ok(())
    }

    fn run_udp(&self) -> std::io::Result<()> {
        // Bind to an ephemeral local port
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        // Connect the socket to the remote server address
        socket.connect(&self.address)?;
        log::info!("Connected to the server: {}", self.address);

        // Call the UDP handler.
        // Since the socket is already connected, it immediately enters the interactive session.
        connection::udp::handle(socket)
    }
}
