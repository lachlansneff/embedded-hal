//! Asynchronous APIs
//!
//! This traits use `core::future::Future` and generic associated types.

pub mod i2c;
pub mod rng;
pub mod serial;
pub mod spi;
pub mod timer;
