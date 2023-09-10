//! ISO-8583 message parsing and formatting.

use std::sync::Arc;

use chrono::Datelike;
use iso8583_rs::iso8583::iso_spec::{new_msg, IsoMsg, Spec};
use tracing::debug;

use op_core::bank_account::models::BankAccount;
use op_core::{
    bank_account::{models::BankAccountUpdate, traits::BankAccountTrait},
    error::DomainError,
    transaction::{models::TransactionCreate, traits::TransactionTrait},
    types::TransactionType,
};

use super::types::*;
use crate::constants::{POPULATED_ISO_MSG_FIELD_NUMBERS, RESPONSE_CODE_FIELD_NUMBER};

#[derive(Clone)]
pub struct Iso8583MessageProcessor {
    /// Spec for the ISO-8583 message
    pub spec: &'static Spec,
    /// Bank account controller
    pub bank_account_controller: Arc<dyn BankAccountTrait>,
    /// Transaction controller
    pub transaction_controller: Arc<dyn TransactionTrait>,
}

impl Iso8583MessageProcessor {
    /// Process the encoded ISO-8583 message and return the response
    pub async fn process(&self, msg: &mut Vec<u8>) -> Result<(Vec<u8>, IsoMsg), DomainError> {
        match self.spec.parse(msg) {
            Ok(iso_msg) => {
                debug!("parsed incoming request - message = \"{}\" successfully. \n : parsed message: \n --- \n {} \n ----\n",
                       iso_msg.msg.name(), iso_msg);

                let req_msg_type = iso_msg.get_field_value(&"message_type".to_string())?;

                let res_msg_type = match req_msg_type.as_str().try_into() {
                    Ok(MTI::AuthorizationRequest) => MTI::AuthorizationResponse,
                    Ok(MTI::ReversalRequest) => MTI::FinancialResponse,
                    _ => {
                        return Err(DomainError::ApiError(
                            "Unsupported message type".to_string(),
                        ))
                    }
                };

                // Create a new response message
                let mut res_iso_msg = new_msg(
                    &iso_msg.spec,
                    iso_msg.spec.get_message_from_header(res_msg_type.into())?,
                );

                // don't copy the fields that we have already set
                res_iso_msg.echo_from(&iso_msg, &POPULATED_ISO_MSG_FIELD_NUMBERS[1..])?;

                // handle authorization request
                match req_msg_type
                    .as_str()
                    .try_into()
                    .expect("Validated above; qed")
                {
                    MTI::AuthorizationRequest => {
                        self.handle_authorization_request(&mut res_iso_msg).await?
                    }
                    MTI::ReversalRequest => self.handle_reversal_request(&mut res_iso_msg).await?,
                    _ => {
                        return Err(DomainError::ApiError(
                            "Unsupported message type".to_string(),
                        ))
                    }
                };

                if let Ok(res_data) = res_iso_msg.assemble() {
                    return Ok((res_data, res_iso_msg));
                }

                Err(DomainError::ApiError(
                    "Failed to assemble new ISO message".to_string(),
                ))
            }
            Err(e) => {
                debug!("Failed to parse incoming request - message = \"{}\". \n : error: \n --- \n {} \n ----\n",
                       String::from_utf8_lossy(msg), e);
                Err(DomainError::ApiError(
                    "Failed to parse incoming request".to_string(),
                ))
            }
        }
    }

    /// Handle authorization request
    ///
    /// Extracts necessary fields from the ISO message and performs authorization.
    pub async fn handle_authorization_request(
        &self,
        iso_msg: &mut IsoMsg,
    ) -> Result<(), DomainError> {
        iso_msg.set(
            &"message_type".to_string(),
            MTI::AuthorizationResponse.into(),
        )?;

        debug!(
            "Bmp child value: {:?} is on {}",
            iso_msg.bmp,
            iso_msg.bmp.is_on(2)
        );
        // extract necessary fields from the ISO message
        let card_number = iso_msg.bmp_child_value(4)?;

        if let Ok(Some(bank_account)) = self
            .bank_account_controller
            .find_by_card_number(&card_number)
            .await
        {
            let validation_result = self
                .validate_with_bank_account(iso_msg, &bank_account)
                .await?;

            // early return if not approved
            if validation_result != ResponseCodes::Approved {
                iso_msg.set_on(RESPONSE_CODE_FIELD_NUMBER, validation_result.into())?;
                return Ok(());
            }

            let amount: u32 = iso_msg
                .get_field_value(&"amount".to_string())?
                .trim()
                .parse()?;

            // perform the transaction
            let update_account = BankAccountUpdate {
                id: bank_account.id,
                amount,
                transaction_type: TransactionType::Credit,
            };

            if let Ok(updated_bank_account) = self
                .bank_account_controller
                .update(&bank_account.id, &update_account)
                .await
            {
                debug!(
                    "Transaction successful, updated bank account: {:?}",
                    updated_bank_account
                );

                // Insert the transaction into the database
                self.transaction_controller
                    .create(&TransactionCreate::new(
                        bank_account.id,
                        None, // TODO: set the receiving bank account
                        amount,
                        TransactionType::Credit,
                        bank_account.nonce,
                        iso_msg.assemble().expect("should be working"),
                    ))
                    .await?;

                iso_msg.set_on(RESPONSE_CODE_FIELD_NUMBER, ResponseCodes::Approved.into())?;
            } else {
                iso_msg.set_on(
                    RESPONSE_CODE_FIELD_NUMBER,
                    ResponseCodes::InvalidTransaction.into(),
                )?;
            }
        } else {
            iso_msg.set_on(
                RESPONSE_CODE_FIELD_NUMBER,
                ResponseCodes::InvalidCardNumber.into(),
            )?;
        }

        Ok(())
    }

    /// Handle reversal request
    ///
    /// Extracts necessary fields from the ISO message and performs reversal.
    pub async fn handle_reversal_request(&self, iso_msg: &mut IsoMsg) -> Result<(), DomainError> {
        iso_msg.set(&"message_type".to_string(), MTI::ReversalResponse.into())?;

        // extract transaction hash from the ISO message
        // first 64 characters are the hash
        let private_data = iso_msg.get_field_value(&"private_data".to_string())?;

        let (tx_hash, _) = private_data.split_at(64);

        let validation_result = self.validate(&iso_msg).await?;

        // early return if not approved
        if validation_result != ResponseCodes::Approved {
            iso_msg.set_on(RESPONSE_CODE_FIELD_NUMBER, validation_result.into())?;
            return Ok(());
        }

        if let Ok(Some(transaction)) = self.transaction_controller.find_by_hash(tx_hash).await {
            // perform the transaction
            let update_account = BankAccountUpdate {
                id: transaction.from,
                amount: transaction.amount,
                transaction_type: TransactionType::Debit,
            };

            if let Ok(updated_bank_account) = self
                .bank_account_controller
                .update(&transaction.from, &update_account)
                .await
            {
                debug!(
                    "Transaction successful, updated bank account: {:?}",
                    updated_bank_account
                );
                // Insert the transaction into the database
                self.transaction_controller
                    .create(&TransactionCreate::new(
                        transaction.from,
                        None, // TODO: set the receiving bank account
                        transaction.amount,
                        TransactionType::Debit,
                        updated_bank_account.nonce,
                        iso_msg.assemble().expect("should be working"),
                    ))
                    .await?;

                iso_msg.set_on(RESPONSE_CODE_FIELD_NUMBER, ResponseCodes::Approved.into())?;
            } else {
                iso_msg.set_on(
                    RESPONSE_CODE_FIELD_NUMBER,
                    ResponseCodes::InvalidTransaction.into(),
                )?;
            }
        } else {
            iso_msg.set_on(
                RESPONSE_CODE_FIELD_NUMBER,
                ResponseCodes::InvalidTransaction.into(),
            )?;
        }

        Ok(())
    }

    /// Validate Iso8583 message
    ///
    /// Does same sanity checks
    async fn validate(&self, iso_msg: &IsoMsg) -> Result<ResponseCodes, DomainError> {
        // extract necessary fields from the ISO message
        let card_number = iso_msg.get_field_value(&"card_number".to_string())?;

        if let Ok(Some(bank_account)) = self
            .bank_account_controller
            .find_by_card_number(&card_number)
            .await
        {
            self.validate_with_bank_account(iso_msg, &bank_account)
                .await
        } else {
            Ok(ResponseCodes::InvalidCardNumber)
        }
    }

    /// Same as [`self.validate`] but with already queried [`BankAccount`]
    async fn validate_with_bank_account(
        &self,
        iso_msg: &IsoMsg,
        bank_account: &BankAccount,
    ) -> Result<ResponseCodes, DomainError> {
        let card_expiration = iso_msg.get_field_value(&"card_expiration".to_string())?;
        let transaction_timestamp =
            iso_msg.get_field_value(&"transaction_timestamp".to_string())?;
        let cvv = iso_msg.get_field_value(&"cvv".to_string())?;

        let amount: u32 = iso_msg
            .get_field_value(&"amount".to_string())?
            .trim()
            .parse()?;

        // validate the transaction timestamp
        // MMDDHHMMSS format, parse it
        if transaction_timestamp.len() != 10 {
            return Ok(ResponseCodes::InvalidTransaction);
        }

        // validate the card expiration date
        let (year, month) = card_expiration.split_at(2);

        if year != bank_account.card_expiration_date.year().to_string()
            || month == bank_account.card_expiration_date.month().to_string()
        {
            return Ok(ResponseCodes::ExpiredCard);
        }

        // validate the CVV
        if cvv != bank_account.card_cvv {
            return Ok(ResponseCodes::DoNotHonor);
        }

        // validate the amount
        if amount > bank_account.balance {
            return Ok(ResponseCodes::InsufficientFunds);
        }

        return Ok(ResponseCodes::Approved);
    }
}
