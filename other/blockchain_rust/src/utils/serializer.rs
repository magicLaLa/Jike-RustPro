use anyhow::Result;
use crypto::{digest::Digest, sha3::Sha3};
use serde::{Deserialize, Serialize};

use crate::error::BlockchainError;

pub fn serialize<T>(data: &T) -> Result<Vec<u8>, BlockchainError>
where
    T: Serialize + ?Sized,
{
    Ok(bincode::serialize(data)?)
}

pub fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, BlockchainError>
where
    T: Deserialize<'a> + ?Sized,
{
  Ok(bincode::deserialize(data)?)
}

pub fn hash_to_str(data: &[u8]) -> String {
  let mut hasher = Sha3::sha3_256();
  hasher.input(data);
  hasher.result_str()
}

#[allow(dead_code)]
pub fn hash_to_u8(data: &[u8], out: &mut [u8]) {
  let mut haser = Sha3::sha3_256();
  haser.input(data);
  haser.result(out);
}
