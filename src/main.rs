use std::{
    net::{self, TcpListener},
    thread,
};

mod cli;
mod connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    // Parse the command-line arguments
    let args = cli::parse();

    // Decide run mode based on `listen` flag
    if args.listen {
        run_server(&args)?;
    } else {
        run_client(&args)?;
    }

    Ok(())
}

/// Run the application in client mode (i.e. connects to the remote server)
fn run_client(args: &cli::Args) -> std::io::Result<()> {
    // Connect to the address
    let stream = net::TcpStream::connect(&args.address)?;
    log::info!("Connected to the server: {}", &args.address);

    // Handle the connection
    connection::handle(stream)?;

    Ok(())
}

/// Run the application in server mode (i.e. listens for incoming connections)
fn run_server(args: &cli::Args) -> std::io::Result<()> {
    // Setup the [`TcpListener`] and bind it to the address
    let listener = TcpListener::bind(&args.address)?;

    log::info!("Server listening on {}...", &args.address);
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
