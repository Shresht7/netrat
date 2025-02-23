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
        // Bind to an ephemeral port on localhost
        let socket = UdpSocket::bind("127.0.0.1:0")?;
        // Connect sets a default destination, which simplifies sending
        socket.connect(&self.address)?;
        log::info!("Connected to the server: {}", &self.address);

        socket.send(b"Hello via UDP!")?;

        Ok(())
    }
}
