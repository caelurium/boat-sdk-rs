//! Versioned sandbox environments, variables, secret files, and repositories.
//! Reads require `environment.read`; mutations require `environment.write`
//! and access to the target environment. Environment contents may include secrets. Changes create
//! a new version; explicitly upgrade existing sandboxes to apply it to them.

mod api;
pub mod models;
