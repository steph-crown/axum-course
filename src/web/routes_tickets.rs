use axum::{
  Json, Router,
  extract::{FromRef, Path, State},
  routing::{delete, get, post},
};
use serde_json::{Value, json};

use crate::{
  Logger, Result,
  model::{ModelController, Ticket, TicketForCreate},
};

#[derive(Clone, FromRef)]
struct AppState {
  mc: ModelController,
}

async fn create_ticket(
  State(mc): State<ModelController>,
  Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
  Logger::info("HANDLER", "create_ticket");
  let ticket = mc.create_ticket(ticket_fc).await?;

  Ok(Json(ticket))
}

async fn list_tickets(mc: State<ModelController>) -> Result<Json<Vec<Ticket>>> {
  Logger::info("HANDLER", "list_tickets");
  let tickets = mc.list_tickets().await?;

  Ok(Json(tickets))
}

async fn delete_ticket(mc: State<ModelController>, Path(id): Path<u64>) -> Result<Json<Ticket>> {
  Logger::info("HANDLER", "delete_ticket");
  let ticket = mc.delete_ticket(id).await?;

  Ok(Json(ticket))
}

pub fn routes(mc: ModelController) -> Router {
  Router::new()
    .route("/tickets", post(create_ticket).get(list_tickets))
    .route("/tickets/{id}", delete(delete_ticket))
    .with_state(mc)
}
