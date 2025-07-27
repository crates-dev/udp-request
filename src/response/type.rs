use crate::*;

/// Boxed trait object for UDP responses.
///
/// Can contain either text or binary response implementations.
pub type BoxResponseTrait =
    Box<dyn ResponseTrait<OutputText = UdpResponseText, OutputBinary = UdpResponseBinary>>;
