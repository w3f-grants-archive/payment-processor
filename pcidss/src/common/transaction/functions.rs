//! Functions for transaction domain.

use super::{
    models::{Transaction, TransactionCreate},
    traits::TransactionTrait,
};
use crate::common::error::DomainError;
use std::sync::Arc;

/// Create a new transaction.
pub async fn create(
    transaction_trait: Arc<dyn TransactionTrait>,
    transaction: TransactionCreate,
) -> Result<Transaction, DomainError> {
    transaction_trait.create(&transaction).await
}

/// Find a transaction by unique identifier.
pub async fn find_by_id(
    transaction_trait: Arc<dyn TransactionTrait>,
    id: uuid::Uuid,
) -> Result<Option<Transaction>, DomainError> {
    transaction_trait.find_by_id(&id).await
}

/// Find a transaction by hash.
pub async fn find_by_hash(
    transaction_trait: Arc<dyn TransactionTrait>,
    hash: String,
) -> Result<Option<Transaction>, DomainError> {
    transaction_trait.find_by_hash(&hash).await
}
