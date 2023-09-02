use std::env;
use std::error::Error;

use deadpool_postgres::Pool;
use deadpool_postgres::{Manager, ManagerConfig, RecyclingMethod};

#[cfg(test)]
use super::error::DomainError;
#[cfg(test)]
use tokio_postgres::NoTls;

struct PostgresConfig {
    host: String,
    user: String,
    password: String,
    name: String,
    pool_max: usize,
}
impl PostgresConfig {
    fn from_env() -> Self {
        Self {
            host: env::var("DATABASE_HOST").expect("DATABASE_HOST must be set"),
            user: env::var("DATABASE_USER").expect("DATABASE_USER must be set"),
            password: env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set"),
            name: env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"),
            pool_max: env::var("DATABASE_POOL_MAX")
                .expect("DATABASE_POOL_MAX must be set")
                .parse::<usize>()
                .expect("DATABASE_POOL_MAX is usize"),
        }
    }
}
impl From<PostgresConfig> for tokio_postgres::Config {
    fn from(_: PostgresConfig) -> Self {
        let mut pg_config = tokio_postgres::Config::new();

        let postgres_config = PostgresConfig::from_env();
        pg_config.user(&postgres_config.user);
        pg_config.password(&postgres_config.password);
        pg_config.dbname(&postgres_config.name);
        pg_config.host(&postgres_config.host);

        pg_config
    }
}

/// Initializes Postgres database
pub fn init() -> Result<deadpool_postgres::Pool, Box<dyn Error>> {
    let postgres_config = PostgresConfig::from_env();
    let pool_max = postgres_config.pool_max;
    let pg_config: tokio_postgres::Config = postgres_config.into();

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let mgr = Manager::new(pg_config, tokio_postgres::NoTls);

    Ok(Pool::builder(mgr).max_size(pool_max).build()?)
}

#[cfg(test)]
pub async fn init_to_tests() -> Result<(), DomainError> {
    let postgres_config = PostgresConfig::from_env();

    let mut pg_config = tokio_postgres::Config::new();
    pg_config.user(&postgres_config.user);
    pg_config.password(&postgres_config.password);
    pg_config.host(&postgres_config.host);

    let (client, connection) = pg_config.connect(NoTls).await?;

    let handler = tokio::spawn(async move {
        connection.await.unwrap();
    });

    let stmt = client.prepare("select datname from pg_database;").await?;
    let result = client.query(&stmt, &[]).await?;
    let mut databases: Vec<String> = vec![];
    result
        .into_iter()
        .for_each(|row| databases.push(row.get("datname")));

    if databases.contains(&postgres_config.name) {
        let stmt = client
            .prepare(&format!("drop database {};", postgres_config.name))
            .await?;
        client.execute(&stmt, &[]).await?;
    }

    let stmt = client
        .prepare(&format!("create database {};", postgres_config.name))
        .await?;
    client.execute(&stmt, &[]).await?;

    handler.abort();

    Ok(())
}
