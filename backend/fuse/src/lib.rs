#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

pub mod algorithm;
pub mod config;
pub mod fuseable;
pub mod types;

pub mod utils;

#[cfg(test)]
mod tests;
