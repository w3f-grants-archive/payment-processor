//! Mock implementation of the Oracle API server.
//!

use std::sync::Arc;

use deadpool_postgres::Pool;
use op_api::{bank_account::PgBankAccount, transaction::PgTransaction};
use op_core::{
    bank_account::traits::BankAccountTrait, postgres::mock_init,
    transaction::traits::TransactionTrait,
};
use tokio::sync::OnceCell;

use crate::iso8583::Iso8583MessageProcessor;

static INIT_DB: OnceCell<Pool> = OnceCell::const_new();

async fn get_db() -> Pool {
    INIT_DB
        .get_or_init(|| async { mock_init().await.expect("Error to init database to tests") })
        .await
        .clone()
}

/// Mock implementation of the Oracle API server.
pub struct MockProcessorImpl {
    /// ISO8583 message processor
    pub processor: Arc<Iso8583MessageProcessor>,
}

impl MockProcessorImpl {
    /// Creates a new instance of the mock processor
    pub async fn new() -> Self {
        let pg_pool = get_db().await;
        let pg_pool = Arc::new(pg_pool);

        let bank_account_trait: Arc<dyn BankAccountTrait> =
            Arc::new(PgBankAccount::new(pg_pool.clone()));
        let transaction_trait: Arc<dyn TransactionTrait> =
            Arc::new(PgTransaction::new(pg_pool.clone()));

        std::env::set_var("SPEC_FILE", "./src/tests/test_spec.yaml");

        let iso8583_spec = iso8583_rs::iso8583::iso_spec::spec("");

        let processor = Iso8583MessageProcessor {
            spec: iso8583_spec,
            bank_account_controller: bank_account_trait,
            transaction_controller: transaction_trait,
        };

        Self {
            processor: Arc::new(processor),
        }
    }
}
