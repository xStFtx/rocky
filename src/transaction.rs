use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Transaction {
    from_address: String,
    to_address: String,
    amount: f64, // Consider using a decimal type for financial calculations
}

impl Transaction {
    /// Creates a new transaction, returns an error if any validation fails.
    pub fn new(from_address: String, to_address: String, amount: f64) -> Result<Transaction, &'static str> {
        if amount <= 0.0 {
            return Err("Amount must be positive");
        }
        if from_address.is_empty() || to_address.is_empty() {
            return Err("From and To addresses must be non-empty");
        }
        Ok(Transaction { from_address, to_address, amount })
    }

    // Getters
    pub fn from_address(&self) -> &str {
        &self.from_address
    }

    pub fn to_address(&self) -> &str {
        &self.to_address
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    // Setters
    pub fn set_from_address(&mut self, from_address: String) {
        self.from_address = from_address;
    }

    pub fn set_to_address(&mut self, to_address: String) {
        self.to_address = to_address;
    }

    pub fn set_amount(&mut self, amount: f64) -> Result<(), &'static str> {
        if amount <= 0.0 {
            return Err("Amount must be positive");
        }
        self.amount = amount;
        Ok(())
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Transaction from {} to {} for {}", self.from_address, self.to_address, self.amount)
    }
}
