use crate::*;

impl ResponseTrait for UdpResponseBinary {
    type OutputText = UdpResponseText;
    type OutputBinary = UdpResponseBinary;

    #[inline]
    fn from(response: &[u8]) -> Self
    where
        Self: Sized,
    {
        response.to_vec()
    }

    #[inline]
    fn binary(&self) -> Self::OutputBinary {
        self.clone()
    }

    #[inline]
    fn text(&self) -> UdpResponseText {
        let data: String = String::from_utf8_lossy(&self).to_string();
        data
    }
}
