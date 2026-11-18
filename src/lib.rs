//! An independent Rust SDK for the Boat VM API.
//!
//! Use [`Client::new`] for the hosted API, or [`Config`] to select a base URL
//! and organization. Credentials are supplied by the caller.

#![forbid(unsafe_code)]

pub mod access;
pub mod account;
pub mod agents;
pub mod ai_credentials;
pub mod api_keys;
mod client;
pub mod commands;
mod config;
pub mod environments;
mod error;
pub mod files;
pub mod sandbox;
mod secret;
pub mod snapshots;

pub use client::Client;
pub use config::Config;
pub use error::Error;
pub use secret::SecretString;
