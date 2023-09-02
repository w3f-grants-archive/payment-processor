//! Models to represent a bank account and its operations.

use chrono::{DateTime, Months, Utc};
use uuid::Uuid;

use crate::common::{error::DomainError, types::TransactionType};

/// `BankAccountCreate` is a model for creating a bank account.
#[derive(Debug, Clone)]
pub struct BankAccountCreate {
    /// Unique identifier of the bank account.
    pub id: Uuid,
    /// Card number linked to the bank account, should be 16 digits.
    pub card_number: String,
    /// Card holder first name.
    pub card_holder_first_name: String,
    /// Card holder last name.
    pub card_holder_last_name: String,
    /// Card expiration date.
    pub card_expiration_date: DateTime<Utc>,
    /// Card CVV.
    pub card_cvv: String,
}

impl BankAccountCreate {
    /// Creates a new `BankAccountCreate`.
    pub fn new(
        card_number: String,
        card_holder_first_name: String,
        card_holder_last_name: String,
        card_cvv: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            card_number,
            card_holder_first_name,
            // expiration date is 4 years from now
            card_expiration_date: Utc::now()
                .checked_add_months(Months::new(48))
                .expect("valid date"),
            card_holder_last_name,
            card_cvv,
        }
    }
}

/// `BankAccountUpdate` is a model for updating a bank account.
#[derive(Debug, Clone)]
pub struct BankAccountUpdate {
    /// Unique identifier of the bank account.
    pub id: Uuid,
    /// Amount of change to the balance.
    pub amount: u32,
    /// Type of change to the balance.
    pub transaction_type: TransactionType,
}

impl BankAccountUpdate {
    /// Creates a new `BankAccountUpdate`.
    pub fn new(id: Uuid, amount: u32, transaction_type: TransactionType) -> Self {
        Self {
            id,
            amount,
            transaction_type,
        }
    }
}

/// Extremely simplified, dummy version of a bank account model.
#[derive(Debug, Clone)]
pub struct BankAccount {
    /// Unique identifier of the bank account.
    pub id: Uuid,
    /// Card number linked to the bank account, should be 16 digits.
    pub card_number: String,
    /// Card holder first name.
    pub card_holder_first_name: String,
    /// Card holder last name.
    pub card_holder_last_name: String,
    /// Card expiration date.
    pub card_expiration_date: DateTime<Utc>,
    /// Card CVV.
    pub card_cvv: String,
    /// Balance of the bank account.
    pub balance: u32,
    /// Nonce of the bank account.
    pub nonce: u32,
}

impl BankAccount {
    /// Creates a new `BankAccount`.
    pub fn new(
        card_number: String,
        card_holder_first_name: String,
        card_holder_last_name: String,
        card_expiration_date: DateTime<Utc>,
        card_cvv: String,
        balance: u32,
        nonce: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            card_number,
            card_holder_first_name,
            card_holder_last_name,
            card_expiration_date,
            card_cvv,
            balance,
            nonce,
        }
    }

    /// Try updating bank account balance
    ///
    /// Simple balance update, no transaction history.
    pub async fn try_update(
        &mut self,
        bank_account_update: &BankAccountUpdate,
    ) -> Result<(), DomainError> {
        self.balance = match bank_account_update.transaction_type.clone() {
            TransactionType::Debit => self.balance.checked_add(bank_account_update.amount),
            TransactionType::Credit => self.balance.checked_sub(bank_account_update.amount),
        }
        .ok_or(DomainError::ApiError(String::from(
            "Arithmetic underflow/overflow",
        )))?;

        self.nonce
            .checked_add(1)
            .ok_or(DomainError::ApiError(String::from(
                "Arithmetic underflow/overflow",
            )))?;

        Ok(())
    }
}
