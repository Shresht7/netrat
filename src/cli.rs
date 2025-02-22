use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// The address to connect to
    pub address: SocketAddr,
}

/// Parses the command-line arguments into a [struct][Args]
pub fn parse() -> Args {
    Args::parse()
}
