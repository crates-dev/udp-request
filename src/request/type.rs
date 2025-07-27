use crate::*;

/// Result type for UDP requests.
///
/// Contains either a boxed response trait object or request error.
pub type RequestResult = Result<BoxResponseTrait, RequestError>;

/// Boxed trait object for UDP request.
///
/// Used to store different request implementations.
pub type BoxRequestTrait = Box<dyn RequestTrait<RequestResult = RequestResult>>;
