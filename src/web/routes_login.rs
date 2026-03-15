use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  password: String,
}

async fn api_login(Json(payload): Json<LoginPayload>) -> Result<Json<Value>> {
  println!("->> {:<12} - api_login", "HANDLER");

  // Todo: implement logic
  if payload.username != "steph" || payload.password != "fuck" {
    return Err(Error::LoginFail);
  }

  // todo: set cookies

  // create uccess.
  let body = Json(json!({
    "result": {
      "success": true
    }
  }));

  Ok(body)
}

pub fn routes() -> Router {
  Router::new().route("/api/login", post(api_login))
}
