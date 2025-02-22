use std::net::SocketAddr;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Connect to a remote server (client mode)
    Connect {
        /// The server address to connect to (e.g. 127.0.0.1:8080)
        address: SocketAddr,
    },

    /// Listen for incoming connections (server mode)
    Listen {
        /// The local address port to bind to (e.g. 4321)
        port: u16,
    },

    /// Scan ports on a host (port scanning mode)
    Scan {
        /// The host to scan (IP Address or Hostname)
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Start scanning from port number
        #[arg(default_value_t = 1)]
        start: u16,

        /// End scanning at port number
        #[arg(default_value_t = 1024)]
        end: u16,
    },
}

/// Parses the command-line arguments into a [struct][Args]
pub fn parse() -> Args {
    Args::parse()
}
