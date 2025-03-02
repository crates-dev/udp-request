#[derive(Debug)]
pub enum Error {
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
