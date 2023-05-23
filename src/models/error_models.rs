use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum VaderError<'a> {
    EventNotActive(&'a str),
    EventNotAdded(&'a str),
    EventEnded(&'a str),
    EventActive(&'a str),
    EventTypeMismatch(&'a str),
    SqlxError(sqlx::Error),
}

impl<'a> From<sqlx::Error> for VaderError<'a> {
    fn from(value: sqlx::Error) -> Self {
        Self::SqlxError(value)
    }
}
impl<'a> Error for VaderError<'a> {}

impl<'a> Display for VaderError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaderError::EventNotAdded(e) => write!(
                f,
                "No Added Event Available for operation.\n[error] :  {}",
                e
            ),
            VaderError::EventNotActive(e) => write!(
                f,
                "No Event is Active to perform operation.\n[error] :  {}",
                e
            ),
            VaderError::SqlxError(e) => write!(f, "Sqlx Error.\n[error] : {}", e),
            VaderError::EventTypeMismatch(e) => write!(
                f,
                "Operation Cannot be performed on this Event Type.\n[error] : {}",
                e
            ),
            VaderError::EventActive(e) => write!(
                f,
                "Operation Cannot be performed on Active Event.\n[error] : {}",
                e
            ),
            VaderError::EventEnded(e) => write!(
                f,
                "Operation Cannot be performed on Event that Ended.\n[error] : {}",
                e
            ),
        }
    }
}
