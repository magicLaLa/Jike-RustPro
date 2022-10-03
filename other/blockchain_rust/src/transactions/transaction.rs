use serde::{Deserialize, Serialize};

use crate::{
    utils::{hash_to_str, serialize},
    Storage, Txinput, Txoutput, UTXOSet,
};

const SUBSIDY: i32 = 10;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Transaction {
    id: String,
    vin: Vec<Txinput>,
    vout: Vec<Txoutput>,
}

impl Transaction {
    pub fn new_coinbase(to: &str) -> Self {
        let txin = Txinput::default();
        let txout = Txoutput::new(SUBSIDY, to);

        let mut tx = Transaction {
            id: String::new(),
            vin: vec![txin],
            vout: vec![txout],
        };

        tx.set_hash();
        tx
    }

    pub fn new_utxo<T: Storage>(from: &str, to: &str, amount: i32, utxo_set: &UTXOSet<T>) -> Self {
        let (accmoulated, valid_outputs) = utxo_set.find_spendable_outputs(from, amount);
        if accmoulated < amount {
            panic!("Error not enough funds");
        }

        let mut inputs = vec![];
        for (txid, outputs) in valid_outputs {
            for idx in outputs {
                let input = Txinput::new(txid.clone(), idx, from.to_string());
                inputs.push(input);
            }
        }

        let mut outputs = vec![Txoutput::new(amount, to)];
        if accmoulated > amount {
            outputs.push(Txoutput::new(accmoulated - amount, from));
        }

        let mut tx = Transaction {
            id: String::new(),
            vin: inputs,
            vout: outputs,
        };

        tx.set_hash();
        tx
    }

    pub fn set_hash(&mut self) {
        if let Ok(tx_er) = serialize(self) {
            self.id = hash_to_str(&tx_er)
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_vout(&self) -> &[Txoutput] {
        self.vout.as_slice()
    }

    pub fn get_vin(&self) -> &[Txinput] {
        self.vin.as_slice()
    }
}
