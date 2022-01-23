use anyhow::Result;
use serde_yaml::Value;
use tokio::{fs, try_join};

#[tokio::main]
async fn main() -> Result<()> {
  let t1 = fs::read_to_string("advanced/38_async/Cargo.toml");
  let t2 = fs::read_to_string("Cargo.lock");

  let (content1, content2) = try_join!(t1, t2)?;

  let yaml1 = toml2yaml(&content1)?;
  let yaml2 = toml2yaml(&content2)?;

  let t3 = fs::write("advanced/38_async/tmp/Cargo.yml", &yaml1);
  let t4 = fs::write("advanced/38_async/tmp/Cargo.lock", &yaml2);

  try_join!(t3, t4)?;

  println!("{}", yaml1);
  println!("{}", yaml2);
  Ok(())
}

fn toml2yaml(content: &str) -> Result<String> {
  let value: Value = toml::from_str(&content)?;
  Ok(serde_yaml::to_string(&value)?)
}