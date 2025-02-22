use std::{
    io::{self, Read, Write},
    net::{Shutdown, TcpStream},
    thread,
};

/// The size of the buffer to use when reading data from stdin or the tcp stream
const BUFFER_SIZE: usize = 1024;

/// Starts a thread that continuously reads from the TCP stream and writes to stdout
pub fn start_reader(mut read_stream: TcpStream) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut stdout = io::stdout().lock(); // Obtain a reference to stdout
        let mut buffer = [0u8; BUFFER_SIZE]; // Create a buffer to hold read data

        loop {
            // Read the stream into the buffer
            let bytes_read = match read_stream.read(&mut buffer) {
                Ok(0) => {
                    // If no bytes were read, then the connection was closed, so we break out of the loop and let the thread drop
                    log::info!("Reader: connection closed by remote");
                    // Shutdown the read half to signal the connection is ending
                    read_stream
                        .shutdown(Shutdown::Read)
                        .expect("failed to shutdown reader");
                    break;
                }

                Ok(n) => n, // Return the number of bytes read

                Err(e) => {
                    log::error!("Reader: error reading from socket: {}", e);
                    break;
                }
            };

            // Write the entire buffer out to stdout
            stdout
                .write_all(&buffer[..bytes_read])
                .expect("failed to write the buffer to stdout");
            stdout.flush().expect("failed to flush stdout");
        }
    })
}

/// Starts a thread that reads from stdin and writes to the TCP stream
pub fn start_writer(mut write_stream: TcpStream) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut stdin = io::stdin().lock(); // Obtain a reference to stdin
        let mut buffer = [0u8; BUFFER_SIZE]; // Create a buffer to hold write data

        loop {
            // Read stdin into the buffer
            let bytes_read = match stdin.read(&mut buffer) {
                Ok(0) => {
                    // End of Input
                    log::info!("Writer: EOF on stdin");
                    // Shutdown the write half to signal end of transmission
                    write_stream
                        .shutdown(Shutdown::Write)
                        .expect("failed to shutdown writer");
                    break;
                }

                Ok(n) => n, // Return the number of bytes read

                Err(e) => {
                    log::error!("Error reading from stdin: {}", e);
                    break;
                }
            };

            // Write the entire buffer out to the stream
            write_stream
                .write_all(&buffer[..bytes_read])
                .expect("failed to write to socket");
        }
    })
}
