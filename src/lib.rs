//! udp-request
//!
//! A simple UDP request library for sending and receiving UDP packets,
//! designed to handle network communication in Rust applications.

mod common;
mod request;
mod response;

pub use {request::*, response::*};

use common::*;

use std::{
    error::Error as StdError,
    fmt::Debug,
    fmt::{self, Display},
    net::UdpSocket,
    sync::{Arc, RwLock},
    time::Duration,
};

#[cfg(test)]
use std::{
    sync::Mutex,
    thread::{JoinHandle, spawn},
    time::Instant,
};
