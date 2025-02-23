# `netrat`

A low-effort `netcat` clone written in Rust.

- **TCP**: Standard TCP communication
- **UDP**: A single bi-directional channel 
- **Port Scanner**: A simple scanner for open ports

## 📦 Installation

<!-- TOOD: Add installation instructions -->

## 📘 Usage

`netrat --help` for full help

### Connect to a TCP server

```sh
netrat connect 127.0.0.1:4321
```

### Start a TCP server

```sh
netrat listen --port 4321 --host 127.0.0.1
```

### Connect to a UDP server

```sh
netrat connect 127.0.0.1:4321 --protocol udp
```

### Start a UDP server

```sh
netrat listen --port 4321 --protocol udp
```

---

## 🖥️ Development

> [!NOTE]
>
> I've decided **not** to implement anything that does any kind of execution. (e.g. `--exec` or reverse shell)

### Project Structure

- `src/main.rs`
    
    The entry point of the command-line application.

- `src/commands/`

    Contains the command implementations:
    - `connect.rs`: Client mode for TCP/UDP connections
    - `listen.rs`: Server mode for TCP/UDP connections

- `src/lib/connections/`

    Contains the core networking logic:
    - `tcp.rs`: Handles interactive TCP sessions.
    - `udp.rs`: Handles interactive UDP sessions with single-connection behavior.

- `src/lib/helpers`

    Utility modules, including:
    - `address.rs`: Parses and forms socket address from client input strings.
    - `protocol.rs`: Defines supported protocols (TCP, UDP)

---

## 📄 License

This project is licensed under the MIT License. See [LICENSE](./LICENSE) for details.
