use std::{
    io::{Read, Write},
    net::UdpSocket,
    thread,
};

// -------
// HANDLER
// -------

/// Handles a UDP session.
///
/// In server mode (when the socket is not yet connected), this function waits for the first datagram,
/// records the client's address, connects the socket to that address, and then launches an interactive session.
///
/// In client mode (when the socket is already connected), it immediately starts the interactive session.
pub fn handle(socket: UdpSocket) -> std::io::Result<()> {
    // Check if the socket is already connected
    // For a connected UDP socket, peer_addr() returns Ok(address)
    match socket.peer_addr() {
        Ok(peer) => {
            log::info!("UDP socket is pre-connected to {}", peer);
            start_interactive_session(socket)
        }
        Err(_) => {
            // Server mode: Wait for the first datagram
            let mut buffer = [0u8; BUFFER_SIZE];

            log::info!("Waiting for a client to connect...");
            let (n, client_addr) = socket.recv_from(&mut buffer)?;
            log::info!(
                "Received initial datagram from {}. Establishing connection...",
                client_addr
            );

            // Write the received data to stdout
            std::io::stdout().write_all(&buffer[..n])?;
            std::io::stdout().flush()?;

            // "Connect" the socket so that subsequent send/recv use the client address
            socket.connect(client_addr)?;

            // Start the interactive session
            start_interactive_session(socket)
        }
    }
}

/// Runs an interactive session over a connected UDP socket.
/// This spawns two threads:
///   - One for reading from the socket and writing to stdout.
///   - One for reading from stdin and sending data to the socket.
fn start_interactive_session(socket: UdpSocket) -> std::io::Result<()> {
    log::info!("Connection established");

    // Split the stream so that we can read and write across threads
    let socket_reader = socket.try_clone()?;
    let socket_writer = socket;

    // Thread to handle incoming data (socket -> stdout)
    let reader_thread_handle = start_reader(socket_reader);
    // Thread to handle outgoing data (stdin -> socket)
    let writer_thread_handle = start_writer(socket_writer);

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

/// Spawns a thread that continuously receives datagrams from the UDP socket and write
fn start_reader(read_socket: UdpSocket) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut stdout = std::io::stdout().lock(); // Obtain a reference to stdout
        let mut buffer = [0u8; BUFFER_SIZE]; // Create a buffer to hold read data

        loop {
            match read_socket.recv(&mut buffer) {
                Ok(0) => {
                    log::info!("Reader: received 0 bytes, ending session.");
                    break;
                }
                Ok(n) => {
                    if let Err(e) = stdout.write_all(&buffer[..n]) {
                        log::error!("Reader: error writing to stdout: {}", e);
                        break;
                    }
                    if let Err(e) = stdout.flush() {
                        log::error!("Reader: error flushing stdout: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    log::error!("Reader: error reading from socket: {}", e);
                    break;
                }
            }
        }
    })
}

/// Spawns a thread that reads from stdin and sends each read chunk as a datagram via the UDP socket
fn start_writer(socket: UdpSocket) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut stdin = std::io::stdin().lock(); // Obtain a reference to stdin
        let mut buffer = [0u8; BUFFER_SIZE]; // Create a buffer to hold write data

        loop {
            match stdin.read(&mut buffer) {
                Ok(0) => {
                    log::info!("Writer: EOF on stdin, ending session.");
                    break;
                }
                Ok(n) => {
                    if let Err(e) = socket.send(&buffer[..n]) {
                        log::error!("Writer: error sending data: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    log::error!("Writer: error reading from stdin: {}", e);
                    break;
                }
            }
        }
    })
}
