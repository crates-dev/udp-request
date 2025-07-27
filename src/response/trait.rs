use crate::*;

/// Trait for UDP response operations.
pub trait ResponseTrait: Send + Debug {
    /// Associated type for text output.
    type OutputText: Clone + Sized;
    /// Associated type for binary output.
    type OutputBinary: Clone + Sized;

    /// Converts response to text format.
    ///
    /// # Returns
    ///
    /// - `Self::OutputText` - Text representation of response.
    fn text(&self) -> Self::OutputText;

    /// Gets binary representation of response.
    ///
    /// # Returns
    ///
    /// - `Self::OutputBinary` - Binary representation of response.
    fn binary(&self) -> Self::OutputBinary;

    /// Creates response from byte slice.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Response data.
    ///
    /// # Returns
    ///
    /// - `Self` - New response instance.
    fn from(response: &[u8]) -> Self
    where
        Self: Sized;
}
