//! PCIDSS Gateway entry point.
use std::{error::Error, sync::Arc};

use deadpool_postgres::Pool;
use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties,
};

use crate::{
    common::{bank_account::traits::BankAccountTrait, transaction::traits::TransactionTrait},
    controllers::{bank_account::PgBankAccount, transaction::PgTransaction},
    pcidss_gateway::{config::get_config, iso8583::Iso8583MessageProcessor},
};

/// Run ISO8583 Message Processor
pub async fn run(pg_pool: Arc<Pool>) -> Result<(), Box<dyn Error>> {
    log::info!("[PCIDSS Gateway] Starting ISO8583 processor");

    let config = get_config();

    // Prepare ISO8583 processor
    let iso8583_spec = iso8583_rs::iso8583::iso_spec::spec("");

    let bank_account_trait: Arc<dyn BankAccountTrait> =
        Arc::new(PgBankAccount::new(pg_pool.clone()));
    let transaction_trait: Arc<dyn TransactionTrait> =
        Arc::new(PgTransaction::new(pg_pool.clone()));

    // Message processor
    let processor = Iso8583MessageProcessor {
        spec: iso8583_spec,
        bank_account_controller: bank_account_trait.clone(),
        transaction_controller: transaction_trait.clone(),
    };

    log::info!("[PCIDSS Gateway] ISO8583 processor initialized");
    log::info!("[PCIDSS Gateway] Starting queue and server listener for ISO8583");

    // Prepare AMQP connection
    let connection =
        Arc::new(Connection::connect(&config.amqp_addr, ConnectionProperties::default()).await?);

    log::info!("[PCIDSS Gateway] Connected to AMQP broker");

    connection.on_error(|err| {
        log::error!("[PCIDSS Gateway] {}", err);
        std::process::exit(1);
    });

    let channel_iso8583 = connection.create_channel().await?;

    // Declare the queue
    let queue = channel_iso8583
        .queue_declare(
            "iso8583",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    log::info!("[PCIDSS Gateway] Declared queue {:?}", queue);

    let consumer_channel = connection.create_channel().await?;

    let mut consumer = consumer_channel
        .basic_consume(
            "iso8583",
            "consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    log::info!("[PCIDSS Gateway] Server listener for ISO8583 started");

    while let Some(result) = consumer.next().await {
        if let Ok(delivery) = result {
            // Process ISO8583 message
            log::info!(
                "[PCIDSS Gateway] Received ISO8583 message: {:?}",
                delivery.data
            );

            let mut delivery_mut = delivery;
            let processing_result = processor.process(&mut delivery_mut.data).await?;

            log::info!(
                "[PCIDSS Gateway] ISO8583 message processing result: {:?}",
                processing_result.0,
            );

            delivery_mut.data = processing_result.0;

            // Send response
            delivery_mut
                .ack(BasicAckOptions::default())
                .await
                .expect("basic_ack");
        }
    }

    Ok(())
}
