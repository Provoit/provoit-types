#[cfg(feature = "diesel")]
pub mod migrations;
pub mod models;
#[cfg(feature = "diesel")]
/// This module is generated by diesel to check queries at compile time
pub mod schema;
