use crate::store::TicketStore;

pub struct AppState {
    pub store: TicketStore,
}

impl AppState {
    pub fn new() -> Self {
        let store = TicketStore::new();

        Self { store }
    }
}
