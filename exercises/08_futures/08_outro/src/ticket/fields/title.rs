use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TicketTitle(String);

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(TicketTitleError::Empty);
        }

        if value.len() > 50 {
            return Err(TicketTitleError::TooLong);
        }

        Ok(Self(value))
    }
}

impl TryFrom<&String> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

#[derive(Debug, Error, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum TicketTitleError {
    #[error("title cannot be blank")]
    Empty,
    #[error("title cannot be longer than 50 bytes")]
    TooLong,
}
