/// Error types for UDP requests.
#[derive(Debug)]
pub enum RequestError {
    /// Invalid URL format.
    InvalidUrl,
    /// Failed to create UDP socket.
    UdpSocketCreateError,
    /// Failed to connect UDP socket.
    UdpSocketConnectError,
    /// General request error.
    RequestError,
    /// Failed to read from connection.
    ReadConnectionError,
    /// Failed to set read timeout.
    SetReadTimeoutError,
    /// Failed to set write timeout.
    SetWriteTimeoutError,
    /// Failed to read response.
    ReadResponseError,
    /// Failed to send response with error message.
    SendResponseError(String),
}
