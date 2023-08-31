use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
use tokio_postgres::Row;
use uuid::Uuid;

use crate::common::{
    bank_account::{
        model::{BankAccount, BankAccountCreate, BankAccountUpdate},
        traits::BankAccountTrait,
    },
    error::DomainError,
};

/// Type that will be used to interact with the database.
pub struct PgBankAccount {
    pool: Arc<Pool>,
}

impl PgBankAccount {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BankAccountTrait for PgBankAccount {
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<BankAccount>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM bank_account WHERE id = $1")
            .await?;

        if let Some(result) = client.query_opt(&stmt, &[&id]).await? {
            return Ok(Some((&result).into()));
        }

        Ok(None)
    }

    async fn update(
        &self,
        id: &Uuid,
        bank_account_update: &BankAccountUpdate,
    ) -> Result<BankAccount, DomainError> {
        let client = self.pool.get().await?;

        let mut bank_account = &self
            .find_by_id(id)
            .await?
            .ok_or(DomainError::NotFound("Bank account not found".to_string()))?;

        bank_account.try_update(bank_account_update).await?;

        let stmt = client
            .prepare("UPDATE bank_account SET balance = $1 WHERE id = $2 RETURNING *")
            .await?;

        let result = client
            .query_one(&stmt, &[&bank_account.balance, &id])
            .await?;

        Ok((&result).into())
    }

    async fn create(
        &self,
        bank_account_create: &BankAccountCreate,
    ) -> Result<BankAccount, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client
            .prepare(
                "INSERT INTO bank_account (id, card_number, card_holder_first_name, card_holder_last_name, card_expiration_date, card_cvv, balance) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            )
            .await?;

        let row = client
            .query_one(
                &stmt,
                &[
                    &bank_account_create.id,
                    &bank_account_create.card_number,
                    &bank_account_create.card_holder_first_name,
                    &bank_account_create.card_holder_last_name,
                    &bank_account_create.card_expiration_date,
                    &bank_account_create.card_cvv,
                    &0_u32, // Initial balance is 0
                ],
            )
            .await?;

        Ok((&row).into())
    }

    async fn delete(&self, id: &Uuid) -> Result<(), DomainError> {
        let client = self.pool.get().await?;
        let stmt = client
            .prepare("DELETE FROM bank_account WHERE id = $1")
            .await?;
        client.execute(&stmt, &[&id]).await?;
        Ok(())
    }
}

/// Implement `From` trait for `BankAccount` from `Row`.
/// Helps with parsing database results.
impl From<&Row> for BankAccount {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            card_holder_first_name: row.get("card_holder_first_name"),
            card_holder_last_name: row.get("card_holder_last_name"),
            card_cvv: row.get("card_cvv"),
            card_expiration_date: row.get("card_expiration_date"),
            card_number: row.get("card_number"),
            balance: row.get("balance"),
        }
    }
}
