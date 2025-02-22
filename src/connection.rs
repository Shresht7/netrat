use std::{
    io::{self, Read, Write},
    net::TcpStream,
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
    })
}

pub fn start_writer(mut write_stream: TcpStream) -> thread::JoinHandle<()> {
    thread::spawn(move || {
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
    })
}
