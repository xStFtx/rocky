#[derive(Debug)]
pub struct Transaction {
    from_address: String,
    to_address: String,
    amount: f64,
}

impl Transaction {
    pub fn new(from_address: String, to_address: String, amount: f64) -> Transaction {
        Transaction { from_address, to_address, amount }
    }
}
