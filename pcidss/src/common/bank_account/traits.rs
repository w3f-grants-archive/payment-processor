//! Defines trait for bank account operations.

use async_trait::async_trait;
use uuid::Uuid;

use super::model::{BankAccount, BankAccountCreate, BankAccountUpdate};
use crate::common::error::DomainError;

/// `BankAccountTrait` is a trait for bank account operations.
///
/// This should be implemented by any bank account controller.
#[async_trait]
pub trait BankAccountTrait: Send + Sync {
    /// Find a bank account by unique identifier.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<BankAccount>, DomainError>;

    /// Create a new bank account.
    async fn create(&self, bank_account: &BankAccountCreate) -> Result<BankAccount, DomainError>;

    /// Update a bank account by unique identifier.
    async fn update(
        &self,
        id: &Uuid,
        bank_account: &BankAccountUpdate,
    ) -> Result<BankAccount, DomainError>;

    /// Delete a bank account by unique identifier.
    async fn delete(&self, id: &Uuid) -> Result<(), DomainError>;
}
