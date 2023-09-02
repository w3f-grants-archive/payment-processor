//! Functions for bank account domain.

use std::sync::Arc;

use crate::common::bank_account::models::{BankAccount, BankAccountCreate, BankAccountUpdate};
use crate::common::bank_account::traits::BankAccountTrait;
use crate::common::error::DomainError;

/// Create a new bank account.
pub async fn create(
    bank_account_trait: Arc<dyn BankAccountTrait>,
    bank_account: BankAccountCreate,
) -> Result<BankAccount, DomainError> {
    bank_account_trait.create(&bank_account).await
}

/// Find a bank account by unique identifier.
pub async fn find_by_id(
    bank_account_trait: Arc<dyn BankAccountTrait>,
    id: uuid::Uuid,
) -> Result<Option<BankAccount>, DomainError> {
    bank_account_trait.find_by_id(&id).await
}

/// Update a bank account by unique identifier.
pub async fn update(
    bank_account_trait: Arc<dyn BankAccountTrait>,
    id: uuid::Uuid,
    bank_account_update: BankAccountUpdate,
) -> Result<BankAccount, DomainError> {
    let account_exists = bank_account_trait.find_by_id(&id).await?;

    if account_exists.is_none() {
        return Err(DomainError::NotFound(String::from(
            "Bank account not found",
        )));
    }

    bank_account_trait.update(&id, &bank_account_update).await
}
