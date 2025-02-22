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
    reader_thread_handle.join().expect("reader thread panicked");
    writer_thread_handle.join().expect("writer thread panicked");

    log::info!("Connection closed");
    Ok(())
}
