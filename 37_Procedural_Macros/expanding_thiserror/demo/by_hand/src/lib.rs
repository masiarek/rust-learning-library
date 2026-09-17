//! The same `ConfigError`, with the impls `#[derive(Error)]` writes spelled
//! out by hand. `Debug` is derived here as well, because thiserror never
//! writes it.

use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ConfigError {
    Empty,
    Io(io::Error),
    BadPort { path: PathBuf, cause: ParseIntError },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Empty => f.write_str("the config file is empty"),
            ConfigError::Io(_) => f.write_str("cannot read the config file"),
            ConfigError::BadPort { path, .. } => write!(f, "{} has no valid port", path.display()),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::Empty => None,
            ConfigError::Io(source) => Some(source),
            ConfigError::BadPort { cause, .. } => Some(cause),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(source: io::Error) -> Self {
        ConfigError::Io(source)
    }
}
