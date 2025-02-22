use std::net;

mod cli;
mod connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    // Parse the command-line arguments
    let args = cli::parse();

    // Connect to the address
    let stream = net::TcpStream::connect(&args.address)?;
    log::info!("Connected to the server: {}", &args.address);

    // Split the stream so that we can read and write across threads
    let read_stream = stream.try_clone()?;
    let write_stream = stream;

    // Thread to handle incoming data (socket -> stdout)
    let reader_thread_handle = connection::start_reader(read_stream);
    // Thread to handle outgoing data (stdin -> socket)
    let writer_thread_handle = connection::start_writer(write_stream);

    // Wait for both threads to finish before exiting
    if let Err(e) = reader_thread_handle.join() {
        log::error!("Reader thread panicked: {:?}", e);
    }
    if let Err(e) = writer_thread_handle.join() {
        log::error!("Writer thread panicked: {:?}", e);
    }

    log::info!("Connection closed");
    Ok(())
}
