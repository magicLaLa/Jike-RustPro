use std::fmt;
use async_trait::async_trait;
use thiserror::Error;
use Architecture::{Plug, PlugResult, Pipeline};

struct Context;

#[derive(Debug, Error)]
enum MyError {
  #[error("Not found: {0}")]
  NotFound(&'static str),
}

#[derive(Debug)]
struct Normalizer;
struct SecurityChecker;
struct CacheLoader;
struct CacheWriter;
struct DataLoader;

impl fmt::Display for Normalizer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "Normalizer")
  }
}

impl fmt::Display for SecurityChecker {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "SecurityChecker")
  }
}

impl fmt::Display for CacheLoader {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "CacheLoader")
  }
}

impl fmt::Display for CacheWriter {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "CacheWriter")
  }
}

impl fmt::Display for DataLoader {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "DataLoader")
  }
}

#[async_trait]
impl Plug<Context> for Normalizer {
    async fn call(&self, _ctx: &mut Context) -> PlugResult<Context> {
        PlugResult::Continue
    }
}

#[async_trait]
impl Plug<Context> for SecurityChecker {
  async fn call(&self, _ctx: &mut Context) -> PlugResult<Context> {
    PlugResult::NewPipe(vec![
      Box::new(CacheLoader),
      Box::new(DataLoader),
      Box::new(CacheWriter),
    ])
  }
}

#[async_trait]
impl Plug<Context> for CacheLoader {
  async fn call(&self, _ctx: &mut Context) -> PlugResult<Context> {
    PlugResult::Continue
  }
}

#[async_trait]
impl Plug<Context> for CacheWriter {
  async fn call(&self, _ctx: &mut Context) -> PlugResult<Context> {
    PlugResult::Continue
  }
}

#[async_trait]
impl Plug<Context> for DataLoader {
  async fn call(&self, _ctx: &mut Context) -> PlugResult<Context> {
    PlugResult::Err(Box::new(MyError::NotFound("something")))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut pipeline = Pipeline::new(
    vec![
      Box::new(SecurityChecker),
      Box::new(Normalizer),
    ]
  );
  let mut ctx = Context;
  let result = pipeline.execute(&mut ctx).await;
  println!("{:?}", pipeline.get_execution_log());
  println!("{:?}", result);
  Ok(())
}