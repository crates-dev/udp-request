use crate::*;

/// Configuration for UDP request.
///
/// Contains network settings and timeout configurations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Remote host address.
    pub(crate) host: String,
    /// Remote port number.
    pub(crate) port: usize,
    /// Request timeout in milliseconds.
    pub(crate) timeout: u64,
    /// Buffer size for receiving data.
    pub(crate) buffer_size: usize,
}
