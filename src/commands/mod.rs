use clap::Subcommand;

mod connect;
use connect::Connect;
mod listen;
use listen::Listen;
mod scan;
use scan::Scan;

#[derive(Subcommand)]
pub enum Command {
    Connect(Connect),
    Listen(Listen),
    Scan(Scan),
}
