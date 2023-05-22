use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum VaderError {
    EventNotActive(String),
    EventNotAdded(String),
    SqlxError(sqlx::Error),
}

impl From<sqlx::Error> for VaderError {
    fn from(value: sqlx::Error) -> Self {
        Self::SqlxError(value)
    }
}
impl Error for VaderError {}

impl Display for VaderError {
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
        }
    }
}
