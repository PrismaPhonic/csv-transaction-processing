use std::collections::HashMap;
use serde::Deserialize;

type TransactionID = u32;

// TransactionCache is an in memory cache representing all transactions received by the system.
pub struct TransactionCache {
    // store holds a map from a transactions unique id, to the relevant transaction, providing instant lookup.
    store: HashMap<TransactionID, Transaction>
}

impl TransactionCache {
    pub fn new() -> TransactionCache {
        TransactionCache {
            store: HashMap::new(),
        }
    }

    pub fn insert(&mut self, tx: Transaction) -> &mut TransactionCache {
        // We can only insert withdrawals and deposits. Disputes, resolves and chargebacks
        // all reference a transaction, but are themselves not really transactions.
        match tx.transaction_type {
            TransactionType::Withdrawal | TransactionType::Deposit => {
                self.store.insert(tx.tx_id, tx);
            },
            _ => {},
        }
        self
    }

    pub fn contains_key(&mut self, tx_id: &u32) -> bool {
        self.store.contains_key(tx_id)
    }

    pub fn get_mut(&mut self, tx_id: &u32) -> Option<&mut Transaction> {
        self.store.get_mut(tx_id)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Transaction {
    #[serde(rename(deserialize = "type"))]
    pub transaction_type: TransactionType,
    #[serde(rename(deserialize = "client"))]
    pub client_id: u16,
    #[serde(rename(deserialize = "tx"))]
    pub tx_id: u32,
    #[serde(default)]
    pub amount: Option<f32>,
    #[serde(skip_deserializing)]
    pub disputed: bool,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}