use std::io;

mod common;
mod controllers;
mod pcidss_gateway;

async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init();

    let pg_pool_result = postgres::init();
}
