use std::{
    net::{SocketAddr, TcpListener, TcpStream},
    sync, thread, time,
};

use netrat::connection;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    // Parse the command-line arguments
    let args = cli::parse();

    // Dispatch based on the command
    match args.command {
        cli::Command::Connect { address } => run_client(&address),
        cli::Command::Listen { port } => run_server(port),
        cli::Command::Scan { host, start, end } => scan_ports(&host, start, end),
    }?;

    Ok(())
}

/// Run the application in client mode (i.e. connects to the remote server)
fn run_client(address: &SocketAddr) -> std::io::Result<()> {
    // Connect to the address
    let stream = TcpStream::connect(&address)?;
    log::info!("Connected to the server: {}", &address);

    // Handle the connection
    connection::handle(stream)?;

    Ok(())
}

/// Run the application in server mode (i.e. listens for incoming connections)
fn run_server(port: u16) -> std::io::Result<()> {
    // Form the local address to use for the server
    let address = format!("127.0.0.1:{}", port);

    // Setup the [`TcpListener`] and bind it to the address
    let listener = TcpListener::bind(&address)?;

    log::info!("Server listening on {}...", &address);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = connection::handle(stream) {
                        log::error!("Error handling client: {:?}", e);
                    }
                });
            }
            Err(e) => log::error!("Connection failed: {:?}", e),
        }
    }

    Ok(())
}

/// Scan ports in the range [start, end] on the given host
fn scan_ports(host: &str, start: u16, end: u16) -> std::io::Result<()> {
    println!("Scanning {} from port {} to {}...", host, start, end);

    // Create a channel
    let (tx, rx) = sync::mpsc::channel();

    for port in start..=end {
        let tx = tx.clone();
        let host = host.to_string();
        thread::spawn(move || {
            let timeout = time::Duration::from_millis(200);
            if let Ok(addr) = format!("{}:{}", host, port).parse::<SocketAddr>() {
                if let Ok(stream) = TcpStream::connect_timeout(&addr, timeout) {
                    drop(stream);
                    tx.send(port).expect("failed to send result");
                }
            }
        });
    }
    drop(tx);

    let mut open_ports: Vec<u16> = rx.iter().collect();
    open_ports.sort();

    println!("Open ports on {}:", host);
    open_ports.iter().for_each(|port| println!("{}", port));
    Ok(())
}
