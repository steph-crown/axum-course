use std::net::SocketAddr;

use axum::{
  Router,
  extract::{Path, Query},
  middleware,
  response::{Html, IntoResponse, Response},
  routing::get,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct GetLoveParams {
  name: Option<String>,
}

mod error;
mod model;
mod utils;
mod web;

use crate::web::{AUTH_TOKEN_KEY, PORT};

pub use self::{
  error::{Error, Result},
  utils::Logger,
};

#[tokio::main]
async fn main() {
  let app = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

  // region: --- Start server
  let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
  let listener = TcpListener::bind(addr).await.unwrap();
  println!("Listening on port {PORT}...");

  axum::serve(listener, app).await.unwrap();
}

fn routes_hello() -> Router {
  Router::new()
    .route("/hello", get(get_love))
    .route("/hello/{name}", get(get_love_path))
}

async fn get_love(cookies: Cookies, Query(params): Query<GetLoveParams>) -> impl IntoResponse {
  println!(
    "->> {:<12} - handler_get_love - \n {:<20} - {params:?}",
    "HANDLER", "QUERY PARAMS"
  );

  let user = params.name.as_deref().unwrap_or("World");

  Html(format!(
    "<h1>Hello world, <b>nikafuckatibeyonce damn bro {user} </b></h1>"
  ))
}

async fn get_love_path(
  Query(params): Query<GetLoveParams>,
  Path(name): Path<String>,
) -> impl IntoResponse {
  println!(
    "->> {:<12} - handler_get_love - \n {:<20} - {params:?} \n {:<20} - {name} ",
    "HANDLER", "QUERY PARAMS", "PATH PARAMS"
  );

  let user = params.name.as_deref().unwrap_or("World");

  Html(format!(
    "<h1>Hello world, <b>nikafuckatibeyonce damn bro {user} {name}</b></h1>"
  ))
}

async fn main_response_mapper(cookies: Cookies, res: Response) -> Response {
  Logger::info("RES_MAPPER", "main_response_mapper");
  println!("FUCKKKKKKK: {:#?}", cookies.get(AUTH_TOKEN_KEY));

  res
}

fn routes_static() -> ServeDir {
  ServeDir::new("./")
}
