use crate::accounts::AccountsCache;
use crate::transactions::{TransactionCache, TransactionType, Transaction};

pub struct TransactionService {
    accounts: AccountsCache,
    transactions: TransactionCache,
}

impl TransactionService {
    pub fn new() -> TransactionService {
        TransactionService {
            accounts: AccountsCache::new(),
            transactions: TransactionCache::new(),
        }
    }


    pub fn apply_transaction(&mut self, tx: &Transaction) -> &mut TransactionService {
        self.transactions.insert(tx.clone());

        // If no client account exists yet, let's create it with an initial deposit.
        if !self.accounts.contains_key(&tx.client_id) {
            self.accounts.initialize_account(tx);
            return self;
        }

        match tx.transaction_type {
            TransactionType::Deposit => {
                self.handle_deposit(tx);
            },
            TransactionType::Withdrawal => {
                self.handle_withdrawal(tx);
            },
            TransactionType::Dispute => {
                self.handle_dispute(tx);
            },
            TransactionType::Resolve => {
                self.handle_resolve(tx);
            },
            TransactionType::Chargeback => {
                self.handle_chargeback(tx);
            },
        }

        self
    }

    pub fn print_accounts(&self) -> String {
        self.accounts.to_string()
    }

    fn handle_deposit(&mut self, tx: &Transaction) -> &mut TransactionService {
        let account = self.accounts.get_mut(&tx.client_id).unwrap();

        // If the clients account is locked we should bail.
        if account.locked {
            // TODO: Return error here.
            return self;
        }

        account.apply_deposit(tx.amount.unwrap());

        self
    }

    fn handle_withdrawal(&mut self, tx: &Transaction) -> &mut TransactionService {
        let account = self.accounts.get_mut(&tx.client_id).unwrap();

        // If the clients account is locked we should bail.
        if account.locked {
            // TODO: Return error here.
            return self;
        }

        account.apply_withdrawal(tx.amount.unwrap());

        self
    }

    fn handle_dispute(&mut self, tx: &Transaction) -> &mut TransactionService {
        let account = self.accounts.get_mut(&tx.client_id).unwrap();

        // If the clients account is locked we should bail.
        if account.locked {
            // TODO: Return error here.
            return self;
        }

        // Find transaction in question. If it doesn't exist, assume partner side error.
        if !self.transactions.contains_key(&tx.tx_id) {
            // TODO: Throw partner-side error.
            return self;
        }

        let disputed_tx = self.transactions.get_mut(&tx.tx_id).unwrap();
        disputed_tx.disputed = true;

        account.apply_dispute(disputed_tx.amount.unwrap());

        self
    }

    fn handle_resolve(&mut self, tx: &Transaction) -> &mut TransactionService {
        let account = self.accounts.get_mut(&tx.client_id).unwrap();

        // If the clients account is locked we should bail.
        if account.locked {
            // TODO: Return error here.
            return self;
        }

        // Find transaction in question. If it doesn't exist, assume partner side error.
        if !self.transactions.contains_key(&tx.tx_id) {
            // TODO: Throw partner-side error.
            return self;
        }

        let resolved_tx = self.transactions.get_mut(&tx.tx_id).unwrap();
        if !resolved_tx.disputed {
            // TODO: Return error.
            return self;
        }

        // Using a mutable reference so no need to re-insert.
        account.apply_resolve(resolved_tx.amount.unwrap());

        resolved_tx.disputed = false;

        self
    }

    fn handle_chargeback(&mut self, tx: &Transaction) -> &mut TransactionService {
        let account = self.accounts.get_mut(&tx.client_id).unwrap();

        // If the clients account is locked we should bail.
        if account.locked {
            // TODO: Return error here.
            return self;
        }

        // Find transaction in question. If it doesn't exist, assume partner side error.
        if !self.transactions.contains_key(&tx.tx_id) {
            // TODO: Throw partner-side error.
            return self;
        }

        let chargeback_tx = self.transactions.get_mut(&tx.tx_id).unwrap();
        if !chargeback_tx.disputed {
            // TODO: Return error.
            return self;
        }

        account.apply_chargeback(chargeback_tx.amount.unwrap());

        chargeback_tx.disputed = false;

        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // This test is flaky because order isn't guaranteed from a hashmap, which means the print results may be out of order.
    // This fits the requirements but makes testing print output harder. One solution would be to either not test print output
    // at all, or use something like IndexMap which guarantees insertion order is respected.
    //
    // Another option could be only passing a portion of the sample data to reflect a single client.
    #[test]
    fn sample_data_passes() {
        let sample_data = "type,client,tx,amount\ndeposit,1,1,1.0\ndeposit,2,2,2.0\ndeposit,1,3,2.0\nwithdrawal,1,4,1.5\nwithdrawal,2,5,3.0";
        let mut rdr = csv::Reader::from_reader(sample_data.as_bytes());
        let mut service = TransactionService::new();
        for result in rdr.deserialize() {
            let transaction: Transaction = result.unwrap();
            // Apply transaction to accounts.
            service.apply_transaction(&transaction);
        }
        let want = "client,available,held,total,locked\n1,1.5000,0.0000,1.5000,false\n2,2.0000,0.0000,2.0000,false\n";
        let got = service.print_accounts();
        assert_eq!(got, want);
    }

    #[test]
    fn dispute_handled_correctly() {
        let sample_data = "type,client,tx,amount\ndeposit,1,1,5.0\ndispute,1,1,\n";
        let mut rdr = csv::Reader::from_reader(sample_data.as_bytes());
        let mut service = TransactionService::new();
        for result in rdr.deserialize() {
            let transaction: Transaction = result.unwrap();
            // Apply transaction to accounts.
            service.apply_transaction(&transaction);
        }
        let want = "client,available,held,total,locked\n1,0.0000,5.0000,5.0000,false\n";
        let got = service.print_accounts();
        assert_eq!(got, want);
    }

    #[test]
    fn chargeback_handled_correctly() {
        let sample_data = "type,client,tx,amount\ndeposit,1,1,5.0\ndispute,1,1,0.0\nchargeback,1,1,\n";
        let mut rdr = csv::Reader::from_reader(sample_data.as_bytes());
        let mut service = TransactionService::new();
        for result in rdr.deserialize() {
            let transaction: Transaction = result.unwrap();
            // Apply transaction to accounts.
            service.apply_transaction(&transaction);
        }
        let want = "client,available,held,total,locked\n1,0.0000,0.0000,0.0000,true\n";
        let got = service.print_accounts();
        assert_eq!(got, want);
    }

    #[test]
    fn resolve_handled_correctly() {
        let sample_data = "type,client,tx,amount\ndeposit,1,1,5.0\ndispute,1,1,0.0\nresolve,1,1,\n";
        let mut rdr = csv::Reader::from_reader(sample_data.as_bytes());
        let mut service = TransactionService::new();
        for result in rdr.deserialize() {
            let transaction: Transaction = result.unwrap();
            // Apply transaction to accounts.
            service.apply_transaction(&transaction);
        }
        let want = "client,available,held,total,locked\n1,5.0000,0.0000,5.0000,false\n";
        let got = service.print_accounts();
        assert_eq!(got, want);
    }
}
