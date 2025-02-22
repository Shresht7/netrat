use std::{
    net::{SocketAddr, TcpStream},
    sync, thread, time,
};

pub struct PortScanner {
    /// The address of the host to scan
    host: String,
    /// The scan starts from this port number
    start: u16,
    /// The scan ends at this port number (inclusive)
    end: u16,
    /// The timeout in milliseconds before considering a port closed when scanning
    timeout: u64,
}

impl PortScanner {
    /// Instantiate a new [`PortScanner`] for the given `host` and range `[start, end]`
    pub fn new(host: &str, start: u16, end: u16) -> Self {
        let host = host.to_owned();
        let timeout = 200;
        Self {
            host,
            start,
            end,
            timeout,
        }
    }

    /// Each port scan will wait for [`timeout`][Self::timeout] milliseconds before considering the port closed
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    /// Scans ports in the range [[`start`][Self::start], [`end`][Self::end]] on the given [`host`][Self::host]
    pub fn scan(&self) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
        // Create a channel to collect open ports from the scanner tasks
        let (tx, rx) = sync::mpsc::channel();

        // Iterate over the range of ports...
        for port in self.start..=self.end {
            let address = format!("{}:{}", self.host, port).parse::<SocketAddr>()?;
            let timeout = time::Duration::from_millis(self.timeout);

            let tx = tx.clone();
            thread::spawn(move || {
                if let Ok(stream) = TcpStream::connect_timeout(&address, timeout) {
                    drop(stream); // We just needed to see if we connect
                    tx.send(port).expect("failed to send result");
                }
            });
        }

        // Note: We have to explicitly `drop` the original `tx` because `rx` will continue to
        // wait for messages as long as it sees that at least one sender (the original `tx` in this case) is still alive.
        // Calling `drop` explicitly closes the original sender here, instead of at the end of the function (which will be too late).
        // This allows `rx` to terminate properly.
        drop(tx);

        // Collect all open ports and return
        let open_ports = rx.iter().collect();
        Ok(open_ports)
    }
}
