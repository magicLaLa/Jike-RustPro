use chrono::prelude::*;
use std::{sync::Arc, thread, time::Duration};
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(2));
    let mut join_handles = Vec::new();

    for item in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
          let local_time = Local::now();
          println!("ind: {:?}, local_time, {:?}", item, local_time);
          thread::sleep(Duration::new(10, 0));
          drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}
