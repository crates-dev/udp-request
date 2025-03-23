use crate::*;

impl ResponseTrait for UdpResponseBinary {
    type OutputText = UdpResponseText;
    type OutputBinary = UdpResponseBinary;

    fn from(response: &[u8]) -> Self
    where
        Self: Sized,
    {
        response.to_vec()
    }

    fn binary(&self) -> Self::OutputBinary {
        self.clone()
    }

    fn text(&self) -> UdpResponseText {
        let data: String = String::from_utf8_lossy(&self).to_string();
        data
    }
}
