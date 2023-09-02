use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref AMQP_CONFIG: Config = Config::from_env();
}

pub fn get_config() -> &'static Config {
    &AMQP_CONFIG
}

/// Configuration for the PCIDSS Gateway.
#[derive(Debug, Clone)]
pub struct Config {
    /// AMQP address
    pub amqp_addr: String,
    /// ISO8583 spec path
    pub iso8583_spec_path: String,
}

impl Config {
    fn from_env() -> Self {
        Self {
            amqp_addr: env::var("AMQP_ADDR").expect("AMQP_ADDR must be set"),
            iso8583_spec_path: env::var("SPEC_FILE").expect("SPEC_FILE must be set"),
        }
    }
}
