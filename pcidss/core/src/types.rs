//! Types used in the library.

/// `TransactionType` is an enum for the type of transaction.
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    /// Add to account balance.
    Debit,
    /// Deduct from account balance.
    Credit,
}

#[allow(clippy::from_over_into)]
impl Into<u32> for TransactionType {
    fn into(self) -> u32 {
        match self {
            TransactionType::Debit => 0,
            TransactionType::Credit => 1,
        }
    }
}
