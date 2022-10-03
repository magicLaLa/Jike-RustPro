use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Txinput {
    txid: String,
    vount: usize,
    from_addr: String,
}

impl Txinput {
    pub fn new(txid: String, vount: usize, from_addr: String) -> Self {
        Self {
            txid,
            vount,
            from_addr: from_addr.into(),
        }
    }

    pub fn can_unlock_output(&self, address: &str) -> bool {
      self.from_addr.eq(address)
    }

    pub fn get_txid(&self) -> String {
      self.txid.clone()
    }

    pub fn get_vount(&self) -> usize {
      self.vount
    }
}
