use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
  const PORT: u16 = 4000;

  // let hc = httpc_test::new_client(format!("http://localhost:{PORT}"))?;
  // hc.do_get("/hello?name=Folaashade").await?.print().await?;

  // // hc.do_get("/src/main.rs").await?.print().await?;

  // let req_login = hc.do_post(
  //   "/api/login",
  //   json!({
  //     "username": "steph",
  //     "password": "fuck"
  //   }),
  // );

  // req_login.await?.print().await?;

  // hc.do_get("/hello?name=Folaashade").await?.print().await?;

  Ok(())
}
