use std::error::Error;

use deadpool_postgres::Manager;
use deadpool_postgres::Pool;

use super::error::DomainError;
use tokio_postgres::NoTls;

pub struct PostgresConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub name: String,
    pub pool_max: usize,
}

impl From<PostgresConfig> for tokio_postgres::Config {
    fn from(value: PostgresConfig) -> Self {
        let mut pg_config = tokio_postgres::Config::new();

        pg_config.user(&value.user);
        pg_config.password(&value.password);
        pg_config.dbname(&value.name);
        pg_config.host(&value.host);

        pg_config
    }
}

/// Initializes Postgres database
pub fn init(postgres_config: PostgresConfig) -> Result<deadpool_postgres::Pool, Box<dyn Error>> {
    let pool_max = postgres_config.pool_max;
    let pg_config: tokio_postgres::Config = postgres_config.into();

    let mgr = Manager::new(pg_config, tokio_postgres::NoTls);

    Ok(Pool::builder(mgr).max_size(pool_max).build()?)
}

pub async fn mock_init() -> Result<Pool, DomainError> {
    use std::env;

    let postgres_config = PostgresConfig {
        host: env::var("POSTGRES_HOST").unwrap_or("localhost".to_string()),
        user: env::var("POSTGRES_USER").unwrap_or("postgres".to_string()),
        password: env::var("POSTGRES_PASSWORD").unwrap_or("postgres".to_string()),
        name: env::var("POSTGRES_NAME").unwrap_or("test".to_string()),
        pool_max: 100,
    };

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

    let mgr = Manager::new(pg_config, tokio_postgres::NoTls);

    Ok(Pool::builder(mgr).max_size(10).build()?)
}
