//! udp-request
//!
//! A simple UDP request library for sending and receiving UDP packets,
//! designed to handle network communication in Rust applications.

#[cfg(test)]
mod cfg;
pub(crate) mod common;
pub(crate) mod request;
pub(crate) mod response;

pub use request::*;
pub use response::*;

pub(crate) use common::*;

pub(crate) use std::{
    error::Error as StdError,
    fmt::Debug,
    fmt::{self, Display},
    net::UdpSocket,
    sync::{Arc, RwLock},
    time::Duration,
};
