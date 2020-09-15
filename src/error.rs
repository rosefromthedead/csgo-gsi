use std::error::Error as StdError;
use std::fmt;

use fehler::throws;

/// any error caused by this library
#[derive(Debug)]
pub enum Error {
    /// an error encountered when trying to install configuration
    ConfigInstallError {
        /// a textual description of the error
        description: &'static str,
        /// an upstream cause of the error
        cause: Option<Box<dyn StdError>>,
    },
}

impl fmt::Display for Error {
    #[throws(fmt::Error)]
    fn fmt(&self, f: &mut fmt::Formatter) {
        match self {
            Error::ConfigInstallError { description, .. } => {
                write!(f, "CS:GO GSI config install error: {}", description)?;
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::ConfigInstallError { cause, .. } => cause.as_deref()
        }
    }
}
