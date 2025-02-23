#[derive(Clone)]
pub enum Protocol {
    TCP,
    UDP,
}

impl std::str::FromStr for Protocol {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tcp" => Ok(Self::TCP),
            "udp" => Ok(Self::UDP),
            _ => Err("Unsupported Protocol".into()),
        }
    }
}
