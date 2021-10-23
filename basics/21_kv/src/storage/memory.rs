use crate::{KvError, Kvpair, Value, Storage};
use dashmap::{mapref::one::Ref, DashMap};

/// 使用 dashmap 构建 Memtable， 实现 Storage trait
#[derive(Debug, Clone, Default)]
pub struct MemTable {
  tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    /// 创建一个 缺省的 MemTable
    pub fn new() -> Self {
      Self::default()
    }

    /// 如果名为 name 的 hash table 不存在，则创建，否则返回
    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
      match self.tables.get(name) {
          Some(table) => table,
          None => {
            let entry = self.tables.entry(name.into()).or_default();
            entry.downgrade()
          }
      }
    }
}

impl Storage for MemTable {
  fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
      let table = self.get_or_create_table(table);
      Ok(table.get(key).map(|v| v.value().clone()))
  }

  fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
      let table = self.get_or_create_table(table);
      Ok(table.insert(key, value))
  }

  fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
    let table = self.get_or_create_table(table);
    Ok(table.contains_key(key))
  }

  fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
    let table = self.get_or_create_table(table);
    Ok(table.remove(key).map(|(_k, v)| v))
  }

  fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
    let table = self.get_or_create_table(table);
    Ok(table
      .iter()
      .map(|v| Kvpair::new(v.key(), v.value().clone()))
      .collect()
    )
  }

  fn get_values(&self, table: &str, keys: &Vec<String>) -> Result<Vec<Option<Value>>, KvError> {
    let table = self.get_or_create_table(table);
    let mut result: Vec<Option<Value>> = Vec::new();
    for key in keys.iter() {
      if let Some(val) = table.get(key).map(|v| v.value().clone()) {
        result.push(Some(val));
      }
    }
    Ok(result)
  }

  fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
      todo!()
  }
}