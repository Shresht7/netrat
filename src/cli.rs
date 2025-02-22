use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// The address to connect to (client mode) or listen on (server mode)
    pub address: SocketAddr,

    /// Listen mode: if set, acts as a server (listens for connections)
    #[clap(short, long)]
    pub listen: bool,
}

/// Parses the command-line arguments into a [struct][Args]
pub fn parse() -> Args {
    Args::parse()
}
