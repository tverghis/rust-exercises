use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TicketStatus {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for TicketStatus {
    type Error = TicketStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Self::ToDo),
            "inprogress" => Ok(Self::InProgress),
            "done" => Ok(Self::Done),
            _ => Err(TicketStatusError::Invalid)
        }
    }
}

#[derive(Debug, Error, Clone, Copy, Eq, PartialEq, Serialize)]
pub enum TicketStatusError {
    #[error("must be one of ToDo, InProgress or Done")]
    Invalid,
}
