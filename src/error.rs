use std::{error::Error as StdError, fmt::Display, result::Result as StdResult};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    CronParseError(cron::error::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CronParseError(e) => e.fmt(f),
        }
    }
}

impl StdError for Error {}

impl From<cron::error::Error> for Error {
    fn from(e: cron::error::Error) -> Self {
        Error::CronParseError(e)
    }
}
