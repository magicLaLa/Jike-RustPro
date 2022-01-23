use anyhow::Result;
use std::time::Duration;

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<()> {
  tokio::spawn(async move {
    eprintln!("task 1");

    tokio::time::sleep(Duration::from_millis(1)).await;
    // loop {}
  });

  tokio::spawn(async move {
    eprintln!("task 2");
  });

  tokio::time::sleep(Duration::from_millis(1)).await;

  tokio::task::spawn(async {
      // ...
      println!("spawned task done!")
  });

  // Yield, allowing the newly-spawned task to execute first.
  tokio::task::yield_now().await;
  println!("main task done!");

  Ok(())
}