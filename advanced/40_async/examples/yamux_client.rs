use anyhow::Result;
use futures::prelude::*;
use tokio::net::TcpStream;
use tokio_util::{
  codec::{Framed, LinesCodec},
  compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt},
};
use tracing::info;
use yamux::{Config, Connection, Mode, WindowUpdateMode};

#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();
  let stream = TcpStream::connect("0.0.0.0:8080").await?;
  info!("Connented to server");
  let mut config = Config::default();
  config.set_window_update_mode(WindowUpdateMode::OnRead);
  let conn = Connection::new(stream.compat(), config, Mode::Client);

  let mut ctrl = conn.control();
  tokio::spawn(
    yamux::into_stream(conn).try_for_each_concurrent(None, |_stream| future::ready(Ok(()))),
  );

  let stream = ctrl.open_stream().await?.compat();
  info!("Started a new stream");
  let mut framed = Framed::new(stream, LinesCodec::new());
  framed.send("Hello this is try".to_string()).await?;
  if let Some(Ok(line)) = framed.next().await {
      println!("Got: {}", line);
  }

  Ok(())
}