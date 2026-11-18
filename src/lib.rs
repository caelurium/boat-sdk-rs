//! An independent Rust SDK for the Boat VM API.
//!
//! Use [`Client::new`] for the hosted API, or [`Config`] to select a base URL
//! and organization. Credentials are supplied by the caller.

#![forbid(unsafe_code)]

mod client;
pub mod commands;
mod config;
mod error;
pub mod sandbox;

pub use client::Client;
pub use config::Config;
pub use error::Error;
