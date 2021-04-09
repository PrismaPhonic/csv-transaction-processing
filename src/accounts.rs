use std::collections::HashMap;
use crate::transactions::{Transaction, TransactionType};

type ClientID = u16;

// AccountsCache is an in memory cache representing the current state of all client accounts within the system.
pub struct AccountsCache {
    // store holds a map from a clients unique id, to their current account state.
    store: HashMap<ClientID, Account>
}

impl AccountsCache {
    pub fn new() -> AccountsCache {
        AccountsCache {
            store: HashMap::new(),
        }
    }

    pub fn contains_key(&mut self, client_id: &u16) -> bool {
        self.store.contains_key(client_id)
    }

    pub fn get_mut(&mut self, client_id: &u16) -> Option<&mut Account> {
        self.store.get_mut(client_id)
    }

    pub fn initialize_account(&mut self, tx: &Transaction) -> &mut AccountsCache {
        if let TransactionType::Deposit = tx.transaction_type {
            let new_account = Account::new(tx.client_id, tx.amount.unwrap());
            self.store.insert(tx.client_id, new_account);
        } else {
            // TODO: Return Error.
        }
        return self
    }
}

// Account represents an individual clients account with the bank.
pub struct Account {
    // client represents the unique client id for the account in question.
    pub client: u16,
    // available represents the total funds that are available for trading, staking, withdrawal, etc.
    pub available: f32,
    // held represents the total funds that are held for dispute.
    pub held: f32,
    // total represents the total funds. It is a sum of the available funds and the held funds.
    pub total: f32,
    // locked represents whether the account is currently locked. An account becomes locked if a charge back occurs.
    pub locked: bool,
}

impl Account {
    // New creates a new Account based on the initial deposit. Because this is a brand new account
    // the funds are available, and no disputes have occurred yet, so the account is not locked and there are
    // no held funds.
    pub fn new(client_id: u16, initial_deposit: f32) -> Account {
        Account {
            client: client_id,
            total: initial_deposit,
            held: 0.0,
            available: initial_deposit,
            locked: false,
        }
    }

    pub fn apply_deposit(&mut self, deposit_amt: f32) -> &mut Account {
        self.total += deposit_amt;
        self.available += deposit_amt;
        self
    }

    pub fn apply_withdrawal(&mut self, withdrawal_amt: f32) -> &mut Account {
        if self.available < withdrawal_amt {
            return self;
        }
        self.total -= withdrawal_amt;
        self.available -= withdrawal_amt;
        self
    }

    pub fn apply_dispute(&mut self, disputed_amt: f32) -> &mut Account {
        self.available -= disputed_amt;
        self.held += disputed_amt;
        self
    }

    pub fn apply_resolve(&mut self, resolve_amt: f32) -> &mut Account {
        self.held -= resolve_amt;
        self.available += resolve_amt;
        self
    }

    pub fn apply_chargeback(&mut self, chargeback_amt: f32) -> &mut Account {
        self.locked = true;
        self.held -= chargeback_amt;
        self.total -= chargeback_amt;
        self
    }
}

impl std::fmt::Display for AccountsCache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = "client,available,held,total,locked\n".to_string();
        for (client_id, account) in &self.store {
            let line = format!("{},{:.4},{:.4},{:.4},{}\n", client_id, account.available, account.held, account.total, account.locked);
            out.push_str(&line);
        }
        write!(f, "{}", out)
    }
}