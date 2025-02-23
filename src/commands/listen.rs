use std::{
    net::{TcpListener, UdpSocket},
    thread,
};

use clap::Parser;

use netrat::helpers::Protocol;

use crate::connection;

/// Listen for incoming connections (server mode)
#[derive(Parser)]
pub struct Listen {
    /// The local address port to bind to (e.g. 4321)
    port: u16,

    /// The local interface to bind to (e.g. 0.0.0.0 for all interfaces, defaults to 127.0.0.1)
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// The protocol to use
    #[arg(short, long, alias = "mode", default_value = "tcp")]
    protocol: Protocol,
}

impl Listen {
    /// Run the application in server mode (i.e. listens for incoming connections)
    pub fn run(&self) -> std::io::Result<()> {
        // Use the host provided, or if "localhost", resolve to "127.0.0.1"
        let host = if self.host == "localhost" {
            "127.0.0.1"
        } else {
            &self.host
        };

        // Form the local address to use for the server
        let address = format!("{}:{}", host, self.port);

        match self.protocol {
            Protocol::TCP => self.run_tcp(address),
            Protocol::UDP => self.run_udp(),
        }
    }

    fn run_tcp(&self, address: String) -> std::io::Result<()> {
        // Setup the [`TcpListener`] and bind it to the address
        let listener = TcpListener::bind(&address)?;

        log::info!("Server listening on {}...", &address);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        if let Err(e) = connection::tcp::handle(stream) {
                            log::error!("Error handling client: {:?}", e);
                        }
                    });
                }
                Err(e) => log::error!("Connection failed: {:?}", e),
            }
        }

        Ok(())
    }

    fn run_udp(&self) -> std::io::Result<()> {
        // Bind the socket to listen on all interfaces at the specified port
        let socket = UdpSocket::bind(&format!("0.0.0.0:{}", self.port))?;
        log::info!("Server listening on 0.0.0.0:{}", self.port);

        // Call the UDP handler.
        // The handler will block until a client sends its first datagram,
        // then record that address and begin the interactive session.
        connection::udp::handle(socket)
    }
}
