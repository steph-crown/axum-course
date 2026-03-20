use std::sync::{Arc, Mutex};

use crate::{Error, Result};
use serde::{Deserialize, Serialize};

// ticket types

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
  pub id: u64,
  title: String,
}

pub struct TicketForCreate {
  title: String,
}

pub struct ModelController {
  tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}
