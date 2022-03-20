use anyhow::Result;
use kv3::{
  start_client_with_config, start_server_with_config, ClientConfig, CommandRequest, ServerConfig,
  StorageConfig,
};
use tokio_rustls::client;
use std::time::Duration;
use tokio::time;

#[tokio::test]
async fn yamux_server_client_full_test() -> Result<()> {
  let addr = "127.0.0.1:10086";

  let mut config: ServerConfig = toml::from_str(include_str!("../fixtures/server.conf"))?;
  config.general.addr = addr.into();
  config.storage = StorageConfig::MemTable;

  tokio::spawn(async move {
    start_server_with_config(&config).await.unwrap();
  });

  time::sleep(Duration::from_millis(10)).await;
  let mut config: ClientConfig = toml::from_str(include_str!("../fixtures/client.conf"))?;
  config.general.addr = addr.into();

  let mut ctrl = start_client_with_config(&config).await.unwrap();
  let mut stream = ctrl.open_stream().await?;

  // 生成一个 HSET 命令
  let cmd = CommandRequest::new_hset("table", "hello", "world".to_string().into());
  stream.execute_unary(&cmd).await?;

  // 生成一个 HEGT 命令
  let cmd  = CommandRequest::new_hget("table", "hello");
  let data = stream.execute_unary(&cmd).await?;

  assert_eq!(data.status, 200);
  assert_eq!(data.values, &["world".into()]);

  Ok(())
}