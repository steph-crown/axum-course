use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::{Error, Result, web::AUTH_TOKEN_KEY};

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
  let auth_token = cookies.get(AUTH_TOKEN_KEY).map(|c| c.value().to_string());

  auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
  Ok(next.run(req).await)
}
