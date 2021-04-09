use std::process;
extern crate csv_transaction_processing;

fn main() {
    let filename = std::env::args().nth(1).expect("no filename provided");
    if let Err(e) = csv_transaction_processing::run(filename) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}