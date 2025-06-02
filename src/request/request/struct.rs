use crate::*;

#[derive(Debug, Clone)]
pub struct UdpRequest {
    pub(crate) config: ArcRwLock<Config>,
    pub(crate) response: ArcRwLock<UdpResponseBinary>,
}
