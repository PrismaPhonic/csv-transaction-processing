
//! # CSV Transaction Processing
//!
//! This is a terminal program that takes a single input of a csv file representing transactions.
//! The format of the csv file you pass in, must be in the form of:
//!
//!    |  type   |   client   |    tx    |  amount  |
//!    | ------- |:----------:|:--------:|:--------:|
//!
//! ## Output
//!
//! This terminal program will output all clients current account state with the following format:
//!
//!    |  client |  available |   held   |  total   |  locked  |
//!    | ------- |:----------:|:--------:|:--------:|:--------:|
//!
//! ## Use
//!
//! To use this terminal program, simply pass it a csv file. It will print the results to stdout:
//!
//! ```terminal
//! $ cargo run -- transactions.csv > accounts.csv
//! ```
//!
//! ## Tests
//!
//! Unit tests have been added in the `src/services.rs` file.
//!
//! ## TODOS:
//! Unfortunately I did not have time to get to appropriate error handling. Ideally I would have built
//! a base error enum and returned specific errors. I've instead left TODOs for now in places where errors
//! should have been returned.
//!
//! Another thing that is less than ideal is that I needed to use an Option type so serde could handle
//! missing amounts for dispute, resolve, and chargeback transaction types. In my program I assume that for
//! withdrawal and deposit they exist, and simply unwrap. I should ideally handle the case explicitly
//! where a withdrawal or deposit line come in with no amount supplied.


use std::error::Error;
use crate::services::TransactionService;
use crate::transactions::Transaction;

mod accounts;
mod services;
mod transactions;

pub fn process_csv(filename: &str) -> Result<String, Box<dyn Error>>{
    let mut rdr = csv::Reader::from_path(filename)?;
    let mut service = TransactionService::new();

    for result in rdr.deserialize() {
        let transaction: Transaction = result?;
        // Apply transaction to accounts.
        service.apply_transaction(&transaction);
    }

    let results = service.print_accounts();

    Ok(results)
}

pub fn run(filename: String) -> Result<(), Box<dyn Error>> {
    let results = process_csv(&filename)?;
    println!("{}", results);
    Ok(())
}