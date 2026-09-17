//! The error type of a program that reads a port number from a config file.
//!
//! This file is all there is: `cargo expand -p config_error` shows every line
//! the derive adds to it.

use std::io;
use std::num::ParseIntError;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("the config file is empty")]
    Empty,
    #[error("cannot read the config file")]
    Io(#[from] io::Error),
    #[error("{path} has no valid port")]
    BadPort {
        path: PathBuf,
        #[source]
        cause: ParseIntError,
    },
}
