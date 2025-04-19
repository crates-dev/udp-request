pub(crate) mod response_binary;
pub(crate) mod response_text;
pub(crate) mod r#trait;
pub(crate) mod r#type;

pub use response_binary::r#type::UdpResponseBinary;
pub use response_text::r#type::UdpResponseText;
pub use r#trait::ResponseTrait;
pub use r#type::BoxResponseTrait;
