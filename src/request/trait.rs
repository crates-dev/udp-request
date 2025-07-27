use crate::*;

/// Trait for UDP request operations.
pub trait RequestTrait: Send + Debug {
    /// Associated result type for request operations.
    type RequestResult: Sized;

    /// Sends data through the request.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Data to send.
    ///
    /// # Returns
    ///
    /// - `Self::RequestResult` - Result of the send operation.
    fn send(&mut self, data: &[u8]) -> Self::RequestResult;
}
