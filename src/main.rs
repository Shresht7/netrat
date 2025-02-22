use std::{
    io::{self, Read, Write},
    net, thread,
};

mod cli;

/// The size of the buffer to use when reading data from stdin or the tcp stream
const BUFFER_SIZE: usize = 1024;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    // Parse the command-line arguments
    let args = cli::parse();

    // Connect to the address
    let stream = net::TcpStream::connect(&args.address)?;
    log::info!("Connected to the server: {}", &args.address);

    // Split the stream so that we can read and write across threads
    let mut read_stream = stream.try_clone()?;
    let mut write_stream = stream;

    // Thread to handle incoming data (socket -> stdout)
    let reader_thread_handle = thread::spawn(move || {
        let mut stdout = io::stdout().lock(); // Obtain a reference to stdout
        let mut buffer = [0u8; BUFFER_SIZE]; // Create a buffer to hold read data
        loop {
            let bytes_read = match read_stream.read(&mut buffer) {
                Ok(0) => break, // If no bytes were read, then the connection was closed, so we break out of the loop to drop the thread
                Ok(n) => n,     // Return the number of bytes read
                Err(e) => {
                    log::error!("Error reading from socket: {}", e);
                    break;
                }
            };
            stdout
                .write_all(&buffer[..bytes_read])
                .expect("failed to write the buffer to stdout");
            stdout.flush().expect("failed to flush stdout");
        }
    });

    // Thread to handle outgoing data (stdin -> socket)
    let writer_thread_handle = thread::spawn(move || {
        let mut stdin = io::stdin().lock(); // Obtain a reference to stdin
        let mut buffer = [0u8; BUFFER_SIZE]; // Create a buffer to hold write data
        loop {
            let bytes_read = match stdin.read(&mut buffer) {
                Ok(0) => break, // End of input
                Ok(n) => n,
                Err(e) => {
                    log::error!("Error reading from stdin: {}", e);
                    break;
                }
            };
            write_stream
                .write_all(&buffer[..bytes_read])
                .expect("failed to write to socket");
        }
    });

    // Wait for both threads to finish before exiting
    reader_thread_handle
        .join()
        .expect("something went wrong in reader thread");
    writer_thread_handle
        .join()
        .expect("something went wrong in writer thread");

    log::info!("Connection closed");
    Ok(())
}
