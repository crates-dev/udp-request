use crate::*;

/// ResponseTrait implementation for binary UDP response.
impl ResponseTrait for UdpResponseBinary {
    type OutputText = UdpResponseText;
    type OutputBinary = UdpResponseBinary;

    /// Creates UdpResponseBinary from byte slice.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Response data.
    ///
    /// # Returns
    ///
    /// - `Self` - New binary response instance.
    fn from(response: &[u8]) -> Self
    where
        Self: Sized,
    {
        response.to_vec()
    }

    /// Gets binary representation of response.
    ///
    /// # Returns
    ///
    /// - `Self::OutputBinary` - Binary response data.
    fn binary(&self) -> Self::OutputBinary {
        self.clone()
    }

    /// Converts binary response to text representation.
    ///
    /// # Returns
    ///
    /// - `UdpResponseText` - Text response data.
    fn text(&self) -> UdpResponseText {
        let data: String = String::from_utf8_lossy(&self).to_string();
        data
    }
}
