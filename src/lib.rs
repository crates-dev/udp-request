#[cfg(test)]
mod cfg;
pub(crate) mod common;
pub(crate) mod request;
pub(crate) mod response;

pub use request::{
    error::r#type::Error as RequestError, r#trait::RequestTrait, r#type::BoxRequestTrait,
    request_builder::r#type::RequestBuilder,
};
pub use response::{
    r#trait::ResponseTrait, r#type::BoxResponseTrait, response_binary::r#type::UdpResponseBinary,
    response_text::r#type::UdpResponseText,
};

pub(crate) use common::{constant::*, r#type::*};
pub(crate) use lombok_macros::*;
pub(crate) use request::{
    config::r#type::*, error::r#type::Error, r#type::RequestResult, request::r#type::*,
};
pub(crate) use std::{
    error::Error as StdError,
    fmt::Debug,
    fmt::{self, Display},
    net::UdpSocket,
    sync::{Arc, RwLock},
    time::Duration,
};
