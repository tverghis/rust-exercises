use std::sync::{Arc, Mutex};

use axum::{extract::{Path, State}, routing::{get, post}, Json, Router};

use crate::{store::CreateTicketError, Ticket};

use super::{schema::{NewTicketRequest, PatchTicketRequest}, state::AppState};

pub fn router(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/tickets/:ticket_id", get(get_ticket_handler).patch(patch_ticket_handler))
        .route("/tickets", post(post_ticket_handler))
        .with_state(state)
}

async fn get_ticket_handler(State(state): State<Arc<Mutex<AppState>>>, Path(ticket_id): Path<usize>) -> Json<Option<Ticket>> {
    let state = state.lock().unwrap();
    let ticket = state.store.get(ticket_id.into());

    Json(ticket.cloned())
}

async fn post_ticket_handler(State(state): State<Arc<Mutex<AppState>>>, Json(body): Json<NewTicketRequest>) -> Result<Json<Ticket>, Json<CreateTicketError>> {
    let mut state = state.lock().unwrap();
    let ticket = state.store.create(body)?;

    Ok(Json(ticket))
}

async fn patch_ticket_handler(State(state): State<Arc<Mutex<AppState>>>, Path(ticket_id): Path<usize>, Json(body): Json<PatchTicketRequest>) -> Result<Json<Option<Ticket>>, Json<CreateTicketError>> {
    let mut state = state.lock().unwrap();
    let ticket = state.store.patch(ticket_id.into(), body)?;

    Ok(Json(ticket.cloned()))
}
