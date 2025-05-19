use crate::*;

#[derive(Debug, Clone, Data)]
pub struct UdpRequest {
    pub(crate) config: ArcRwLock<Config>,
    pub(crate) response: ArcRwLock<UdpResponseBinary>,
}
