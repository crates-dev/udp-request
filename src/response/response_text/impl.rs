use crate::*;

impl ResponseTrait for UdpResponseText {
    type OutputText = UdpResponseText;
    type OutputBinary = UdpResponseBinary;

    #[inline]
    fn from(response: &[u8]) -> Self::OutputText
    where
        Self: Sized,
    {
        <UdpResponseBinary as ResponseTrait>::from(response).text()
    }

    #[inline]
    fn text(&self) -> Self::OutputText {
        self.clone()
    }

    #[inline]
    fn binary(&self) -> UdpResponseBinary {
        self.clone().into_bytes()
    }
}
