use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum VaderError<'a> {
    EventNotActive(&'a str),
    EventEnded(&'a str),
    EventActive(&'a str),
    EventTypeMismatch(&'a str),
    SqlxError(sqlx::Error),
    SqlxFieldError(&'a str),
    EventNotFound(&'a str),
    TeamNotFound(&'a str),
    UserNotFound(&'a str),
}

impl<'a> From<sqlx::Error> for VaderError<'a> {
    fn from(value: sqlx::Error) -> Self {
        Self::SqlxError(value)
    }
}
impl<'a> Error for VaderError<'a> {}

impl<'a> actix_web::ResponseError for VaderError<'a> {}

impl<'a> Display for VaderError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaderError::EventNotActive(e) => write!(
                f,
                "No Event is Active to perform operation.\n[error] :  {}",
                e
            ),
            VaderError::SqlxError(e) => write!(f, "Sqlx Error.\n[error] : {}", e),
            VaderError::SqlxFieldError(e) => write!(f, "Sqlx Field DecodeError.\n[error] : {}", e),
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
            VaderError::EventNotFound(e) => write!(f, "Event not Found.\n[error] : {}", e),
            VaderError::TeamNotFound(e) => write!(f, "Team not Found.\n[error] : {}", e),
            VaderError::UserNotFound(e) => write!(f, "User not Found.\n[error] : {}", e),
        }
    }
}
