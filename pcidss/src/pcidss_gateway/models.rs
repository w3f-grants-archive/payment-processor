use serde::Deserialize;

use crate::common::bank_account::model::{BankAccountCreate, BankAccountUpdate};

/// Message broker always receives ISO8583 type messages.
#[derive(Debug, Deserialize)]
pub struct Iso8583(iso8583_rs::iso8583::iso_spec::IsoMsg);

impl From<Iso8583> for BankAccountCreate {
    fn from(iso_8583_msg: Iso8583) -> BankAccountCreate {}
}
