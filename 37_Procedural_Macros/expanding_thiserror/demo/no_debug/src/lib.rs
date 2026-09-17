//! `#[derive(Error)]` on a type with no `Debug`.

use thiserror::Error;

#[derive(Error)]
#[error("the config file is empty")]
pub struct EmptyConfig;
