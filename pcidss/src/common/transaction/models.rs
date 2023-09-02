//! Models to represent a transaction and its operations.
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::common::types::TransactionType;

#[derive(Debug, Clone)]
pub struct TransactionCreate {
    /// Unique identifier of the bank account.
    pub id: Uuid,
    /// Unique identifier of the sender bank account.
    pub from: Uuid,
    /// Unique identifier of the receiving bank account, if any.
    pub to: Option<Uuid>,
    /// Amount of the transaction.
    pub amount: u32,
    /// Type of the transaction.
    pub transaction_type: TransactionType,
    /// Nonce of the transaction.
    pub nonce: u32,
    /// Raw ISO message of the transaction.
    pub iso_msg_raw: Vec<u8>,
}

impl TransactionCreate {
    /// Creates a new `TransactionCreate`.
    pub fn new(
        from: Uuid,
        to: Option<Uuid>,
        amount: u32,
        transaction_type: TransactionType,
        nonce: u32,
        iso_msg_raw: Vec<u8>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            amount,
            transaction_type,
            nonce,
            iso_msg_raw,
        }
    }
}

/// `Transaction` is a model for a transaction.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Unique identifier of the transaction.
    pub id: Uuid,
    /// Unique hash of the transaction.
    pub hash: String,
    /// Unique identifier of the bank account.
    pub from: Uuid,
    /// Unique identifier of the receiving bank account, if any.
    pub to: Option<Uuid>,
    /// Amount of the transaction.
    pub amount: u32,
    /// Type of the transaction.
    pub transaction_type: u32,
}

impl Transaction {
    /// Creates a new `Transaction`.
    pub fn new(
        id: Uuid,
        hash: String,
        from: Uuid,
        to: Option<Uuid>,
        amount: u32,
        transaction_type: TransactionType,
    ) -> Self {
        Self {
            id,
            hash,
            from,
            to,
            amount,
            transaction_type: transaction_type.into(),
        }
    }
}

impl From<&TransactionCreate> for Transaction {
    fn from(value: &TransactionCreate) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(value.iso_msg_raw.clone());
        hasher.update(value.nonce.to_be_bytes());
        let hash = hasher.finalize();

        Self {
            id: value.id,
            hash: format!("{:x}", hash),
            from: value.from,
            to: value.to,
            amount: value.amount,
            transaction_type: value.transaction_type.clone().into(),
        }
    }
}
