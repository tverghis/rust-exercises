use serde::Serialize;
use thiserror::Error;

use crate::server::schema::{NewTicketRequest, PatchTicketRequest};
use crate::ticket::fields::TicketStatus;

use super::fields::TicketStatusError;
use super::{
    fields::{TicketDescriptionError, TicketId, TicketTitleError},
    Ticket,
};

#[derive(Debug, Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
    last_id: Option<TicketId>,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CreateTicketError {
    #[error("invalid title")]
    InvalidTitle(#[from] TicketTitleError),
    #[error("invalid description")]
    InvalidDescription(#[from] TicketDescriptionError),
    #[error("invalid status: {0}")]
    InvalidStatus(#[from] TicketStatusError),
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
            last_id: None,
        }
    }

    pub fn create(&mut self, new: NewTicketRequest) -> Result<Ticket, CreateTicketError> {
        let id = match self.last_id {
            Some(id) => id.incr(),
            None => 0.into(),
        };

        let title = new.title.try_into()?;
        let description = new.description.try_into()?;
        let status = TicketStatus::ToDo;

        let ticket = Ticket {
            id,
            title,
            description,
            status,
        };

        self.tickets.push(ticket.clone());
        self.last_id = Some(id);

        Ok(ticket)
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        let TicketId(idx) = id;
        self.tickets.get(idx)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        let TicketId(idx) = id;
        self.tickets.get_mut(idx)
    }

    pub fn patch(
        &mut self,
        id: TicketId,
        patch: PatchTicketRequest,
    ) -> Result<Option<&Ticket>, CreateTicketError> {
        let ticket = self.get_mut(id);

        match ticket {
            None => Ok(None),
            Some(ticket) => {
                if let Some(title) = patch.title {
                    ticket.title = title.try_into()?;
                }

                if let Some(description) = patch.description {
                    ticket.description = description.try_into()?;
                }

                if let Some(status) = patch.status {
                    ticket.status = status.try_into()?;
                }

                Ok(Some(ticket))
            }
        }
    }
}
