#[derive(Debug)]
pub enum RequestError {
    InvalidUrl,
    UdpSocketCreateError,
    UdpSocketConnectError,
    RequestError,
    ReadConnectionError,
    SetReadTimeoutError,
    SetWriteTimeoutError,
    ReadResponseError,
    SendResponseError(String),
}
