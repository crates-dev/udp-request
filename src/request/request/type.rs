use crate::*;

#[derive(Debug, Clone, Lombok)]
pub struct UdpRequest {
    pub(crate) data: ArcRwLock<Vec<u8>>,
    pub(crate) config: ArcRwLock<Config>,
    pub(crate) response: ArcRwLock<UdpResponseBinary>,
}
