#[cfg(test)]
mod cfg;
pub(crate) mod common;
pub(crate) mod request;
pub(crate) mod response;

pub use request::{
    error::r#type::Error as RequestError, request_builder::r#type::RequestBuilder,
    r#trait::RequestTrait, r#type::BoxRequestTrait,
};
pub use response::{
    response_binary::r#type::UdpResponseBinary, response_text::r#type::UdpResponseText,
    r#trait::ResponseTrait, r#type::BoxResponseTrait,
};

pub(crate) use common::{r#const::*, r#type::*};
pub(crate) use lombok_macros::*;
pub(crate) use request::{
    config::r#type::*, error::r#type::Error, request::r#type::*, r#type::RequestResult,
};
pub(crate) use std::{
    error::Error as StdError,
    fmt::Debug,
    fmt::{self, Display},
    net::UdpSocket,
    sync::{Arc, RwLock},
    time::Duration,
};
