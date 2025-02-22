use clap::Parser;

use crate::commands::Command;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

/// Parses the command-line arguments into a [struct][Args]
pub fn parse() -> Args {
    Args::parse()
}

impl Args {
    /// Run the command-line application
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Command::Connect(cmd) => cmd.run()?,
            Command::Listen(cmd) => cmd.run()?,
            Command::Scan(cmd) => cmd.run()?,
        };
        Ok(())
    }
}
