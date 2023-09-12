//! Mock implementation of the Oracle API server.

use std::sync::Arc;

use chrono::{Months, Utc};
use deadpool_postgres::Pool;
use op_api::{bank_account::PgBankAccount, transaction::PgTransaction};
use op_core::{
    bank_account::{models::BankAccountCreate, traits::BankAccountTrait},
    postgres::mock_init,
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
#[derive(Clone)]
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

        // insert dev accounts
        for account in DEV_ACCOUNTS.iter() {
            let expiration_date = if account.0 != "Eve" {
                Utc::now()
                    .checked_add_months(Months::new(48))
                    .expect("valid date")
            } else {
                Utc::now()
                    .checked_sub_months(Months::new(2))
                    .expect("safe; qed")
            };

            let bank_account_create = BankAccountCreate {
                id: uuid::Uuid::new_v4(),
                card_number: account.1.to_string(),
                card_holder_first_name: account.0.to_string(),
                card_holder_last_name: account.0.to_string(),
                card_cvv: account.2.to_string(),
                card_expiration_date: expiration_date,
                balance: account.3,
            };

            let bank_account = processor
                .bank_account_controller
                .create(&bank_account_create)
                .await
                .unwrap();

            assert_eq!(bank_account.card_number, account.1);
            assert_eq!(bank_account.balance, account.3);
        }

        Self {
            processor: Arc::new(processor),
        }
    }
}

/// Assert an expression returns an error specified.
///
/// Used as `assert_err!(expression_to_assert, expected_error_expression)`
#[macro_export]
macro_rules! assert_err {
    ( $x:expr , $y:expr $(,)? ) => {
        assert_eq!($x, Err($y.into()));
    };
}

/// Represents truncated version of dev accounts
pub(crate) type DevAccount = (&'static str, &'static str, &'static str, u32);

// Development accounts
pub const DEV_ACCOUNTS: [DevAccount; 6] = [
    // Healthy account
    ("Alice", "4169812345678901", "123", 1000),
    // Zero balance case
    ("Bob", "4169812345678902", "124", 0),
    ("Charlie", "4169812345678903", "125", 12345),
    ("Dave", "4169812345678904", "126", 1000000),
    // Expired card
    ("Eve", "4169812345678905", "127", 1000),
    // Mock acquirer account, i.e merchant
    ("Acquirer", "123456", "000", 1000000000),
];
