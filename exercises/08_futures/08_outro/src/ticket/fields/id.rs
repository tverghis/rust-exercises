use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct TicketId(pub usize);

impl TicketId {
    pub fn incr(&self) -> Self {
        let TicketId(cur_id) = self;
        Self(cur_id + 1)
    }
}

impl From<usize> for TicketId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
