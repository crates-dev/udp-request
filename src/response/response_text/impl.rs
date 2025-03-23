use crate::*;

impl ResponseTrait for UdpResponseText {
    type OutputText = UdpResponseText;
    type OutputBinary = UdpResponseBinary;

    fn from(response: &[u8]) -> Self::OutputText
    where
        Self: Sized,
    {
        <UdpResponseBinary as ResponseTrait>::from(response).text()
    }

    fn text(&self) -> Self::OutputText {
        self.clone()
    }

    fn binary(&self) -> UdpResponseBinary {
        self.clone().into_bytes()
    }
}
