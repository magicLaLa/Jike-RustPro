use anyhow::Result;
use futures::prelude::*;
use tokio::net::TcpListener;
use tokio_util::{
  codec::{Framed, LinesCodec},
  compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt},
};
use tracing::info;
use yamux::{Config, Connection, Mode, WindowUpdateMode};

#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();
  let addr = "0.0.0.0:8080";
  let listen = TcpListener::bind(addr).await?;
  info!("Listening on {:?}", addr);

  loop {
      let (stream, addr) = listen.accept().await?;
      info!("Accepted: {:?}", addr);
      let mut config = Config::default();
      config.set_window_update_mode(WindowUpdateMode::OnRead);
      // 转换
      let conn = Connection::new(stream.compat(), config, Mode::Server);
      // yamux ctrl stream 可以打开新的 stream
      let _ctrl = conn.control();
      tokio::spawn(
        yamux::into_stream(conn).try_for_each_concurrent(None, move|s| async move {
          let mut framed = Framed::new(s.compat(), LinesCodec::new());
          while let Some(Ok(line)) = framed.next().await {
              println!("Got: {}", line);
              framed
                .send(format!("Hello! I got '{}'", line))
                .await
                .unwrap()
          }
          Ok(())
        }),
      );
  }
}