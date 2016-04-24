//!  Several libraries used in solving Project Euler problems (https://projecteuler.net)
//!
//! # Using this library
//!
//! Just add the following to your [`Cargo.toml`](http://crates.io/):
//!
//! ```toml
//! [dependencies]
//! eu_lib = { path = "your/path/to/eu_lib" }
//! ```
//! And add this to your root crate.
//!
//! ```
//! extern crate eu_lib;
//! ```
//!
pub mod cards;
pub mod common;
pub mod primes;
