use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TicketDescription(String);

impl TryFrom<String> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(TicketDescriptionError::Empty);
        }

        if value.len() > 500 {
            return Err(TicketDescriptionError::TooLong);
        }

        Ok(Self(value))
    }
}

impl TryFrom<&String> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = TicketDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

#[derive(Debug, Error, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum TicketDescriptionError {
    #[error("title cannot be blank")]
    Empty,
    #[error("title cannot be longer than 500 bytes")]
    TooLong,
}
