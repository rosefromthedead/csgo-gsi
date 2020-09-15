use std::error::Error as StdError;
use std::fmt;

use fehler::throws;

#[derive(Debug)]
pub enum Error {
    ConfigInstallError {
        description: &'static str,
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
