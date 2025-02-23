use std::{net::TcpStream, thread};

/// Establish a relay connection between a client and a target server
pub fn handle(mut client: TcpStream, mut target: TcpStream) -> std::io::Result<()> {
    // Clone the streams to allow bidirectional copying
    let mut target_stream_clone = target.try_clone()?;
    let mut client_stream_clone = client.try_clone()?;

    // Spawn a thread for copying data from client to target
    let client_to_target = thread::spawn(move || {
        let bytes_relayed = std::io::copy(&mut client, &mut target)
            .expect("Error forwarding data from client to target");
        log::trace!(
            "Relaying information from client to target ({} bytes)",
            bytes_relayed
        );
    });

    // Spawn a thread for copying data from target to client
    let target_to_client = thread::spawn(move || {
        let bytes_relayed = std::io::copy(&mut target_stream_clone, &mut client_stream_clone)
            .expect("Error forwarding data from target to client");
        log::trace!(
            "Relayed information from target to client ({} bytes)",
            bytes_relayed
        );
    });

    // Wait for both threads to complete
    client_to_target
        .join()
        .expect("Client-to-target thread panicked");
    target_to_client
        .join()
        .expect("Target-to-client thread panicked");

    Ok(())
}
