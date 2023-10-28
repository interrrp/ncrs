//! Netcat, but in Rust.
//!
//! This crate is not meant to be used as a library; this `lib.rs` is meant for tests.

pub use args::Args;
pub use client::run_client;
pub use server::run_server;

mod args;
mod client;
mod server;
