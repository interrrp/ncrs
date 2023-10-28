//! Contains the command-line argument definitions ([`Args`]).
//!
//! # Usage
//!
//! Since this is a `clap` parser, you can use it like so:
//!
//! ```rust
//! use netcat::Args;
//!
//! let args = Args::parse();
//! ```
//!
//! [`Args`]: ./struct.Args.html

use clap::Parser;

/// Netcat, but in Rust
#[derive(Parser)]
pub struct Args {
    /// The address to connect to (host:port)
    pub address: String,
}
