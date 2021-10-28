pub mod abi;

use abi::{command_request::RequestData, *};
use http::StatusCode;

use crate::KvError;

impl CommandRequest {
    /// 创建 HEST 命令
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }
    /// 创建 HGET 命令
    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }
    /// 创建 HGETALL 命令
    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }
    /// 创建 HMGET 命令
    pub fn new_hmget(table: impl Into<String>, keys: Vec<impl Into<String> + Copy>)  -> Self {

        Self {
            request_data: Some(RequestData::Hmget(Hmget {
                table: table.into(),
                keys: keys
                    .iter()
                    .map(|&s| s.into())
                    .collect(),
            })),
        }
    }
    /// 创建 HDEL 命令
    pub fn new_hdel(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hdel(Hdel {
                table: table.into(),
                key: key.into(),
            })),
        }
    }
}


/// 从 Value 转换成 CommandResponse
impl From<Value> for CommandResponse {
  fn from(v: Value) -> Self {
      Self {
          status: StatusCode::OK.as_u16() as _,
          values: vec![v],
          ..Default::default()
      }
  }
}

impl From<Option<Value>> for CommandResponse {
    fn from(v: Option<Value>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: match v {
                Some(v) => vec![v],
                None => vec![],
            },
            ..Default::default()
        }
    }
}

impl From<Vec<Option<Value>>> for CommandResponse {
    fn from(v: Vec<Option<Value>>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: v.iter()
                .filter(|b| b.is_some())
                .map(|res| res.clone().unwrap())
                .collect(),
            ..Default::default()
        }
    }
}

/// 从 Vec<Kvpair> 转换成 CommandResponse
impl From<Vec<Kvpair>> for CommandResponse {
  fn from(v: Vec<Kvpair>) -> Self {
      Self {
          status: StatusCode::OK.as_u16() as _,
          pairs: v,
          ..Default::default()
      }
  }
}

/// 从 KvError 转换成 CommandResponse
impl From<KvError> for CommandResponse {
  fn from(e: KvError) -> Self {
      let mut result = Self {
          status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
          message: e.to_string(),
          values: vec![],
          pairs: vec![],
      };

      match e {
          KvError::NotFound(_, _) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
          KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
          _ => {}
      }

      result
  }
}

impl Kvpair {
    /// 创建一个新的 kv pair
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

/// 从 String 转换成 Value
impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}

/// 从 &str 转换成 Value
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.into())),
        }
    }
}

/// 从 i64 转换成 Value
impl From<i64> for Value {
  fn from(s: i64) -> Self {
      Self {
          value: Some(value::Value::Integer(s)),
      }
  }
}
