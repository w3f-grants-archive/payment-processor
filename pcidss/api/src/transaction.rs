use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
use uuid::Uuid;

use op_core::{
    error::DomainError,
    transaction::{
        models::{Transaction, TransactionCreate},
        traits::TransactionTrait,
    },
};

/// Type that will be used to interact with the database.
pub struct PgTransaction {
    pool: Arc<Pool>,
}

impl PgTransaction {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TransactionTrait for PgTransaction {
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Transaction>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM bank_transaction WHERE id = $1")
            .await?;

        if let Some(result) = client.query_opt(&stmt, &[&id]).await? {
            return Ok(Some((&result).into()));
        }

        Ok(None)
    }

    async fn find_by_beneficiary(
        &self,
        beneficiary: &Uuid,
    ) -> Result<Vec<Transaction>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM bank_transaction WHERE beneficiary = $1")
            .await?;

        let result = client.query(&stmt, &[&beneficiary]).await?;

        Ok(result.iter().map(|row| (row).into()).collect())
    }

    async fn find_by_hash(&self, hash: &str) -> Result<Option<Transaction>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM bank_transaction WHERE hash = $1")
            .await?;

        if let Some(result) = client.query_opt(&stmt, &[&hash]).await? {
            return Ok(Some((&result).into()));
        }

        Ok(None)
    }

    async fn create(
        &self,
        transaction_create: &TransactionCreate,
    ) -> Result<Transaction, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client
            .prepare(
                "INSERT INTO bank_transaction (id, hash, beneficiary, recipient, amount, transaction_type) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            )
            .await?;
        let transaction: Transaction = transaction_create.into();

        let row = client
            .query_one(
                &stmt,
                &[
                    &transaction.id,
                    &transaction.hash,
                    &transaction.from,
                    &transaction.to,
                    &(transaction.amount as i32),
                    &(transaction.transaction_type as i32),
                ],
            )
            .await?;

        Ok((&row).into())
    }
}
