use anyhow::Result;
use datafusion::{
  arrow::datatypes::{DataType, Field, Schema, SchemaRef},
  prelude::*,
};
use serde::{Deserialize, Serialize};
use std::{io::{self, BufRead}};
use std::sync::Arc;

const PREFIX: &str = "advanced/44_data_processing";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SchemaDataType {
    /// int 64
    Integer,
    /// Utf8
    String,
    /// Date64
    Date,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SchemaFiled {
  name: String,
  #[serde(rename = "type")]
  pub(crate) data_type: SchemaDataType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SchemaFileds(Vec<SchemaFiled>);

impl From<SchemaDataType> for DataType {
  fn from(dt: SchemaDataType) -> Self {
      match dt {
          SchemaDataType::Integer => Self::Int64,
          SchemaDataType::Date => Self::Date64,
          SchemaDataType::String => Self::Utf8,
      }
  }
}

impl From<SchemaFiled> for Field {
    fn from(f: SchemaFiled) -> Self {
        Self::new(&f.name, f.data_type.into(), false)
    }
}

impl From<SchemaFileds> for SchemaRef {
    fn from(fields: SchemaFileds) -> Self {
        let fields: Vec<Field> = fields.0.into_iter().map(|f| f.into()).collect();
        Arc::new(Schema::new(fields))
    }
}

/// nginx 日志处理的数据结构
pub struct NginxLog {
  ctx: ExecutionContext,
}

impl NginxLog {
  /// 根据 schema 定义，数据文件以及分隔符构建 NginxLog 结构
  pub async fn try_new(schema_file: &str, data_file: &str, delim: u8) -> Result<Self> {
    let content = tokio::fs::read_to_string(schema_file).await?;
    let fields: SchemaFileds = serde_yaml::from_str(&content)?;
    let schema = SchemaRef::from(fields);

    let mut ctx = ExecutionContext::new();
    let options = CsvReadOptions::new()
      .has_header(false)
      .delimiter(delim)
      .schema(&schema);
    ctx.register_csv("nginx", data_file, options).await?;

    Ok(Self { ctx })
  }

  /// 进行 sql 查询
  pub async fn query(&mut self, query: &str) -> Result<Arc<dyn DataFrame>> {
    let df = self.ctx.sql(query).await?;
    Ok(df)
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  let mut nginx_log = NginxLog::try_new(
    &format!("{}/fixtures/log_schema.yml", PREFIX),
    &format!("{}/fixtures/nginx_logs.csv", PREFIX),
  b' ')
  .await?;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  while let Some(Ok(line)) = lines.next() {
    println!("line.starts_with {:?}", line.starts_with("--"));
      if !line.starts_with("--") {
        println!("{}", line);
        // 读到一行 sql，查询，获取 dataframe
        let df = nginx_log.query(&line).await?;
        df.show().await?;
      }
  }

  Ok(())
}