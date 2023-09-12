//! Error types for the domain layer

use std::num::ParseIntError;

use iso8583_rs::iso8583::IsoError;
use log::error;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("{}", _0)]
    NotFound(String),

    #[error("{}", _0)]
    BadRequest(String),

    #[error("{}", _0)]
    InternalServerError(String),

    #[error("{}", _0)]
    ApiError(String),
}

impl From<tokio_postgres::Error> for DomainError {
    fn from(err: tokio_postgres::Error) -> Self {
        DomainError::InternalServerError(err.to_string())
    }
}

impl From<deadpool_postgres::PoolError> for DomainError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        DomainError::InternalServerError(err.to_string())
    }
}

impl From<deadpool_postgres::BuildError> for DomainError {
    fn from(err: deadpool_postgres::BuildError) -> Self {
        DomainError::InternalServerError(err.to_string())
    }
}

impl From<deadpool_postgres::CreatePoolError> for DomainError {
    fn from(err: deadpool_postgres::CreatePoolError) -> Self {
        DomainError::InternalServerError(err.to_string())
    }
}

impl From<IsoError> for DomainError {
    fn from(value: IsoError) -> Self {
        DomainError::ApiError(value.msg)
    }
}

impl From<ParseIntError> for DomainError {
    fn from(value: ParseIntError) -> Self {
        DomainError::InternalServerError(value.to_string())
    }
}
