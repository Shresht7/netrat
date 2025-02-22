//! Helpers for working with socket address in a command-line environment

use std::net::{SocketAddr, ToSocketAddrs};

/// A wrapper around [`SocketAddr`] that provides flexible parsing.
///
/// This type accepts:
/// - A full socket address (e.g. "192.168.1.1:8080").
/// - A port number only (e.g. "8080"), which is interpreted as "127.0.0.1:8080".
/// - "localhost" with a port (e.g. "localhost:8080"), which is converted to "127.0.0.1:8080".
///
/// # Examples
///
/// ```
/// use netrat::helpers::Address;
/// // Parse a full address.
/// let addr: Address = "192.168.1.1:8080".parse().unwrap();
/// assert_eq!(addr.to_string(), "192.168.1.1:8080");
///
/// // Parse a port-only address.
/// let addr: Address = "8080".parse().unwrap();
/// assert_eq!(addr.to_string(), "127.0.0.1:8080");
///
/// // Parse localhost with port.
/// let addr: Address = "localhost:8080".parse().unwrap();
/// assert_eq!(addr.to_string(), "127.0.0.1:8080");
/// ```
#[derive(Clone)]
pub struct Address(SocketAddr);

impl std::str::FromStr for Address {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // If the input can be parsed as a socket address already, do so
        if let Ok(addr) = input.parse::<SocketAddr>() {
            return Ok(Self(addr));
        }

        // See if input contains a :
        if input.contains(":") {
            if let Some((host, port)) = input.split_once(":") {
                // Resolve the host name
                let host = match host == "localhost" || host == "" {
                    true => "127.0.0.1",
                    false => return Err("Could not parse socket host".into()),
                };
                // Resolve the port number
                let port = match port.parse::<u32>() {
                    Ok(port) => port,
                    Err(e) => return Err(format!("{}", e)),
                };
                // Try parsing as socket address now
                if let Ok(addr) = format!("{}:{}", host, port).parse::<SocketAddr>() {
                    return Ok(Self(addr));
                }
            }
        }

        // Check if the input is a number and try forming the socket address by assuming localhost
        if let Ok(port) = input.parse::<u32>() {
            if let Ok(addr) = format!("127.0.0.1:{}", port).parse::<SocketAddr>() {
                return Ok(Self(addr));
            }
        }

        Err("Could not parse as socket address".into())
    }
}

impl AsRef<SocketAddr> for Address {
    fn as_ref(&self) -> &SocketAddr {
        &self.0
    }
}

impl ToSocketAddrs for Address {
    type Iter = std::vec::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        // Simply return an iterator over the inner socket address
        Ok(vec![self.0].into_iter())
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Address;
    use std::net::SocketAddr;

    #[test]
    fn test_full_address() {
        let addr: Address = "192.168.1.1:8080".parse().unwrap();
        assert_eq!(
            addr.as_ref(),
            &"192.168.1.1:8080".parse::<SocketAddr>().unwrap()
        );
    }

    #[test]
    fn test_port_only() {
        let addr: Address = "8080".parse().unwrap();
        assert_eq!(
            addr.as_ref(),
            &"127.0.0.1:8080".parse::<SocketAddr>().unwrap()
        );
    }

    #[test]
    fn test_localhost_with_port() {
        let addr: Address = "localhost:8080".parse().unwrap();
        assert_eq!(
            addr.as_ref(),
            &"127.0.0.1:8080".parse::<SocketAddr>().unwrap()
        );
    }

    #[test]
    fn test_invalid() {
        let result: Result<Address, _> = "not_an_address".parse();
        assert!(result.is_err());
    }
}
