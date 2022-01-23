use anyhow::{anyhow, Result};
use serde_yaml::Value;
use std::{
  fs,
  thread::{self, JoinHandle}, io::Read,
};

/// 包装一下 JoinHandle，可以提供额外方法
struct MyJoinHandle<T>(JoinHandle<Result<T>>);

impl<T> MyJoinHandle<T> {
    pub fn thread_await(self) -> Result<T> {
      self.0.join().map_err(|_| anyhow!("failed"))?
    }
}
fn main() -> Result<()> {
  let t1 = thread_read("advanced/38_async/Cargo.toml");
  let t2 = thread_read("Cargo.lock");

  let content1 = t1.thread_await()?;
  let content2 = t2.thread_await()?;

  let yaml1 = toml2yaml(&content1)?;
  let yaml2 = toml2yaml(&content2)?;

  let t3 = thread_write("advanced/38_async/tmp/Cargo.yml", yaml1);
  let t4 = thread_write("advanced/38_async/tmp/Cargo.lock", yaml2);

  let yaml1 = t3.thread_await()?;
  let yaml2 = t4.thread_await()?;

  fs::write("advanced/38_async/tmp/Cargo.yml", &yaml1);
  fs::write("advanced/38_async/tmp/Cargo.lock", &yaml2);

  println!("{}", yaml1);
  println!("{}", yaml2);
  Ok(())
}

fn thread_read(filename: &'static str) -> MyJoinHandle<String> {
  let handle = thread::spawn(move || {
    let s = fs::read_to_string(filename)?;
    Ok::<_, anyhow::Error>(s)
  });
  MyJoinHandle(handle)
}

fn thread_write(filename: &'static str, content: String) -> MyJoinHandle<String> {
  let handle = thread::spawn(move || {
    fs::write(filename, &content)?;
    Ok::<_, anyhow::Error>(content)
  });
  MyJoinHandle(handle)
}

fn toml2yaml(content: &str) -> Result<String> {
  let value: Value = toml::from_str(&content)?;
  Ok(serde_yaml::to_string(&value)?)
}