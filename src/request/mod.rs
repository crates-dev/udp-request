pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod request;
pub(crate) mod request_builder;
pub(crate) mod r#trait;
pub(crate) mod r#type;

pub use error::r#enum::RequestError;
pub use request_builder::r#struct::RequestBuilder;
pub use r#trait::RequestTrait;
pub use r#type::BoxRequestTrait;

pub(crate) use config::r#struct::*;
pub(crate) use request::r#struct::*;
pub(crate) use r#type::RequestResult;
