//! Account identity, organization membership, and billing context. `/me` and
//! organization discovery use `account.read`; account writes use `account.admin`. The active
//! organization is an account-wide setting; use [`crate::Config::with_organization`]
//! to scope a client without changing that default.

mod api;
pub mod models;
