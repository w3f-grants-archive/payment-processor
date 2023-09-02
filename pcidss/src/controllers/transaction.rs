use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
use uuid::Uuid;

use crate::common::{
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
            .prepare("SELECT * FROM transaction WHERE id = $1")
            .await?;

        if let Some(result) = client.query_opt(&stmt, &[&id]).await? {
            return Ok(Some((&result).into()));
        }

        Ok(None)
    }

    async fn find_by_hash(&self, hash: &str) -> Result<Option<Transaction>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM transaction WHERE hash = $1")
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
                "INSERT INTO transaction (id, hash, from, to, amount, transaction_type) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
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
                    &transaction.amount,
                    &transaction.transaction_type,
                ],
            )
            .await?;

        Ok((&row).into())
    }
}

impl From<&tokio_postgres::Row> for Transaction {
    fn from(row: &tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            hash: row.get("hash"),
            from: row.get("from"),
            to: row.get("to"),
            amount: row.get("amount"),
            transaction_type: row.get("transaction_type"),
        }
    }
}
