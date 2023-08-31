//! ISO-8583 message parsing and formatting.

use iso8583_rs::iso8583::{
    iso_spec::IsoMsg,
    server::{ISOServer, MsgProcessor},
    IsoError,
};
use tracing::{debug, error};

#[derive(Debug, Clone)]
pub enum ResponseCodes {
    // 00 - Approved
    Approved,
    // 12 - Invalid transaction
    InvalidTransaction,
    // 13 - Invalid amount, if it overflows
    InvalidAmount,
    // 14 - Invalid PAN
    InvalidCardNumber,
    // 51 - Insufficient funds, if it underflows
    InsufficientFunds,
    // 54 - Expired card
    ExpiredCard,
}

impl Into<String> for ResponseCodes {
    fn into(self) -> String {
        match self {
            ResponseCodes::Approved => "00".to_string(),
            ResponseCodes::InvalidTransaction => "12".to_string(),
            ResponseCodes::InvalidAmount => "13".to_string(),
            ResponseCodes::InvalidCardNumber => "14".to_string(),
            ResponseCodes::InsufficientFunds => "51".to_string(),
            ResponseCodes::ExpiredCard => "54".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Iso8583MessageProcessor {}

impl MsgProcessor for Iso8583MessageProcessor {
    fn process(
        &self,
        iso_server: &ISOServer,
        msg: &mut Vec<u8>,
    ) -> Result<(Vec<u8>, IsoMsg), IsoError> {
        match iso_server.spec.parse(msg) {
            Ok(iso_msg) => {
                let t1 = std::time::Instant::now();
                debug!("parsed incoming request - message = \"{}\" successfully. \n : parsed message: \n --- \n {} \n ----\n",
                       iso_msg.msg.name(), iso_msg);

                iso_server.txn_rate_metric().mark(1);

                let req_msg_type = iso_msg
                    .get_field_value(&"message_type".to_string())
                    .expect("message_type field not found");

                let res_msg_type = if req_msg_type == "0100" {
                    "0110"
                } else if req_msg_type == "0200" {
                    "0210"
                } else {
                    return Err(IsoError {
                        msg: "Unsupported message type".to_string(),
                    });
                };

                let mut res_iso_msg = iso_server
                    .spec
                    .new_iso_msg(res_msg_type)
                    .expect("Failed to create response message");
            }
            Err(e) => {
                error!("failed to parse incoming request - message = \"{}\". \n : error: \n --- \n {} \n ----\n",
                       iso_server.spec.name(), e);
                return Err(e);
            }
        }
    }
}
