//! Main entrypoint of the oracle

use dotenv::dotenv;
use std::{io, sync::Arc};

use common::postgres;
mod common;
mod controllers;
mod pcidss_gateway;

/// Main entrypoint of the oracle
#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pg_pool_result = postgres::init();
    if pg_pool_result.is_err() {
        log::error!("{}", pg_pool_result.unwrap_err());
        std::process::exit(1)
    }

    let pg_pool = Arc::new(pg_pool_result.unwrap());

    let pg_pool_move = pg_pool.clone();
    tokio::spawn(async move {
        let result = pcidss_gateway::lib::run(pg_pool_move).await;
        if result.is_err() {
            log::error!("{}", result.unwrap_err().to_string());
            std::process::exit(1)
        }
    });

    Ok(())
}
