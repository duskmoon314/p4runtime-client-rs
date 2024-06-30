//! p4runtime-client: A P4Runtime client wrapper crate

#![deny(missing_docs)]

pub mod client;
pub mod config;
pub mod counter;
pub mod digest;
pub mod p4info;
pub mod table;

pub use p4runtime;
