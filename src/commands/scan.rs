use clap::Parser;

use netrat::PortScanner;

#[derive(Parser)]
/// Scan ports on a host (port scanning mode)
pub struct Scan {
    /// The host to scan (IP Address or Hostname)
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Start scanning from port number
    #[arg(default_value_t = 1)]
    start: u16,

    /// End scanning at port number
    #[arg(default_value_t = 1024)]
    end: u16,

    /// The timeout (in milliseconds) before considering a port closed
    #[arg(short, long, default_value_t = 200)]
    timeout: u64,
}

impl Scan {
    /// Scan ports in the range [start, end] on the given host
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "Scanning {} from port {} to {}...",
            self.host, self.start, self.end
        );

        // Initialize the PortScanner and scan for open ports
        let mut open_ports = PortScanner::new(&self.host, self.start, self.end)
            .with_timeout(self.timeout)
            .scan()?;
        open_ports.sort(); // Sort the ports

        println!("Open ports on {}:", self.host);
        for port in open_ports {
            println!("{}", port)
        }
        Ok(())
    }
}
