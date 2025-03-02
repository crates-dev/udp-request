use crate::*;

#[derive(Debug, Clone)]
pub struct RequestBuilder {
    pub(crate) udp_request: UdpRequest,
    pub(crate) builder: UdpRequest,
}
