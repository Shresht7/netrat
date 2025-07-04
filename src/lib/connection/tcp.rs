use std::{
    io::{self, Read, Write},
    net::{Shutdown, TcpStream},
    thread,
};

// -------
// HANDLER
// -------

/// Handles the bi-directional I/O communication on the [stream][TcpStream]
pub fn handle(stream: TcpStream) -> std::io::Result<()> {
    log::info!("Connection established");

    // Split the stream so that we can read and write across threads
    let read_stream = stream.try_clone()?;
    let write_stream = stream;

    // Thread to handle incoming data (socket -> stdout)
    let reader_thread_handle = start_reader(read_stream);
    // Thread to handle outgoing data (stdin -> socket)
    let writer_thread_handle = start_writer(write_stream);

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

// -----
// TASKS
// -----

/// The size of the buffer to use when reading data from stdin or the tcp stream
const BUFFER_SIZE: usize = 1024;

/// Starts a thread that continuously reads from the TCP stream and writes to stdout
fn start_reader(mut read_stream: TcpStream) -> thread::JoinHandle<()> {
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
                    if let Err(e) = read_stream.shutdown(Shutdown::Read) {
                        log::error!("Reader: failed to shutdown read half: {}", e);
                    }

                    break;
                }

                Ok(n) => n, // Return the number of bytes read

                Err(e) => {
                    log::error!("Reader: error reading from socket: {}", e);
                    break;
                }
            };

            // Write the entire buffer out to stdout
            if let Err(e) = stdout.write_all(&buffer[..bytes_read]) {
                log::error!("Reader: error writing to stdout: {}", e);
                break;
            }
            if let Err(e) = stdout.flush() {
                log::error!("Reader: error flushing stdout: {}", e);
                break;
            }
        }
    })
}

/// Starts a thread that reads from stdin and writes to the TCP stream
fn start_writer(mut write_stream: TcpStream) -> thread::JoinHandle<()> {
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
                    if let Err(e) = write_stream.shutdown(Shutdown::Write) {
                        log::error!("Writer: failed to shutdown write half: {}", e);
                    }

                    break;
                }

                Ok(n) => n, // Return the number of bytes read

                Err(e) => {
                    log::error!("Error reading from stdin: {}", e);
                    break;
                }
            };

            // Write the entire buffer out to the stream
            if let Err(e) = write_stream.write_all(&buffer[..bytes_read]) {
                log::error!("Writer: error writing to socket: {}", e);
                break;
            }
        }
    })
}
