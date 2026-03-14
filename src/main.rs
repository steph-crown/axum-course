use std::net::SocketAddr;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
  const PORT: u16 = 3000;
  let app = Router::new().route("/", get(get_love));

  // region: --- Start server
  let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
  let listener = TcpListener::bind(addr).await.unwrap();
  println!("Listening on port {PORT}...");

  axum::serve(listener, app).await.unwrap();
}

async fn get_love() -> String {
  String::from("value")
}
