use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};

use crate::{Error, Logger, Result, web::AUTH_TOKEN_KEY};

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  password: String,
}

async fn api_login(cookies: Cookies, Json(payload): Json<LoginPayload>) -> Result<Json<Value>> {
  // println!("->> {:<12} - api_login", "HANDLER");
  Logger::info("HANDLER", "api_login");
  println!("{:#?}", cookies.get(AUTH_TOKEN_KEY));

  // Todo: implement logic
  if payload.username != "steph" || payload.password != "fuck" {
    return Err(Error::LoginFail);
  }

  // set cookies
  cookies.add(Cookie::new(AUTH_TOKEN_KEY, "user-1.exp.sign"));

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
