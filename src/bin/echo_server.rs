use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    // Retrieve the command-line arguments
    let mut args = std::env::args();
    args.next(); // Consume the first argument (the path to this executable)

    // Extract the port number from the command-line arguments
    let port = args.next().expect("needs a port number").parse::<u32>()?;

    // Form the socket address
    let address: SocketAddr = format!("127.0.0.1:{}", port).parse()?;

    // Create a new TcpListener
    let listener = TcpListener::bind(address)?;

    log::info!("Server listening on {}...", address);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log::info!("Accepted connection: {}", stream.peer_addr()?);

                thread::spawn(move || {
                    let peer_addr = stream
                        .peer_addr()
                        .and_then(|r| Ok(r.to_string()))
                        .unwrap_or_default();

                    if let Err(e) = handle_connection(stream) {
                        log::error!("Error handling client: {} {:?}", peer_addr, e);
                    }

                    log::info!("Closing connection: {}", peer_addr);
                });
            }
            Err(e) => log::error!("Connection failed: {:?}", e),
        }
    }

    Ok(())
}

/// The size of the stream reading buffer
const BUFFER_SIZE: usize = 1024;

/// Handles an individual [connection stream][TcpStream]
fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    // A buffer to hold the incoming stream data
    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            // If no more bytes were read, then the client has closed the connection
            // So we break out of the loop to let the connection drop
            break;
        }
        // Screaming-echo the received data back to the client
        let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_uppercase();
        stream.write_all(response.as_bytes())?;
        stream.flush()?; // Immediately flush the contents
    }

    Ok(())
}
