//! # Netrat Library
//!
//! This library provides core functionality for [`netrat`], including connection handling,
//! helpers (e.g., for parsing socket addresses), and a simple port scanner.
//!
//! ## Modules
//!
//! - [`connection`]: Handles bidirectional I/O between a TCP stream and standard I/O.
//! - [`helpers`]: Contains utility types such as `Address` for working with socket addresses.
//! - [`port_scanner`]: Provides a simple port scanner for checking open TCP ports.

pub mod connection;
pub mod helpers;
mod port_scanner;
pub use port_scanner::*;
