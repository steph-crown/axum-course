use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
  const PORT: u16 = 3000;

  let hc = httpc_test::new_client(format!("http://localhost:{PORT}"))?;
  hc.do_get("/hello/sexy?name=Folaashade")
    .await?
    .print()
    .await?;

  hc.do_get("/src/main.rs").await?.print().await?;

  Ok(())
}
