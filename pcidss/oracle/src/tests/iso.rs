//! ISO-8583 processing tests

use chrono::Months;
use iso8583_rs::iso8583::iso_spec::{new_msg, IsoMsg, Spec};
use op_core::{bank_account::models::BankAccount, transaction::models::Transaction};

use crate::{
    tests::mock::*,
    types::{ResponseCodes, MTI},
};

const ALICE: DevAccount = DEV_ACCOUNTS[0];
const BOB: DevAccount = DEV_ACCOUNTS[1];
const CHARLIE: DevAccount = DEV_ACCOUNTS[2];
const DAVE: DevAccount = DEV_ACCOUNTS[3];
const EVE: DevAccount = DEV_ACCOUNTS[4];
const ACQUIRER: DevAccount = DEV_ACCOUNTS[5];

/// Get bank account by card number
async fn get_bank_account_by_card_number(
    api: &MockProcessorImpl,
    card_number: &str,
) -> BankAccount {
    api.processor
        .bank_account_controller
        .find_by_card_number(card_number)
        .await
        .unwrap()
        .unwrap()
}

/// Creates new mock ISO-8583 message
///
/// # Cases
/// * Eve's card is expired
/// * Bob and Dave have zero balance
/// * Alice and Charlie have healthy accounts
///
/// # Arguments
///
/// * `spec` - ISO-8583 specification
/// * `mti` - Message type indicator
/// * `account` - Dev account
fn get_new_iso_msg(spec: &'static Spec, mti: MTI, account: DevAccount) -> IsoMsg {
    let mut msg = new_msg(
        spec,
        spec.get_message_from_header(mti.clone().into()).unwrap(),
    );
    let (name, card_number, cvv, _) = account;

    msg.set("message_type", mti.into()).unwrap();

    msg.set_on(2, &card_number).unwrap();
    // processing code
    msg.set_on(3, "000000").unwrap();

    let now = chrono::Utc::now();

    // transmission date and time
    msg.set_on(7, &format!("{}", now.format("%m%d%H%M%S")))
        .unwrap();

    // time date
    msg.set_on(12, &format!("{}", now.format("%H%M%S")))
        .unwrap();
    // month day
    msg.set_on(13, &format!("{}", now.format("%m%d"))).unwrap();

    // card expiration date
    let exp_date = if name == "Eve" {
        now.checked_sub_months(Months::new(2))
            .expect("safe; qed")
            .format("%m%y")
            .to_string()
    } else {
        now.checked_add_months(Months::new(48))
            .expect("valid date")
            .format("%m%y")
            .to_string()
    };

    msg.set_on(14, &exp_date).unwrap();
    msg.set_on(18, "4816").unwrap();
    msg.set_on(32, "123456").unwrap();
    msg.set_on(35, &format!("{}D{}C{}", card_number, exp_date, cvv))
        .unwrap();
    msg.set_on(41, "12345678").unwrap();
    msg.set_on(42, "ABCDEFGH_000001").unwrap();
    msg.set_on(43, "Dummy business name, Dummy City, 1200000")
        .unwrap();
    msg.set_on(49, "997").unwrap();
    msg.set_on(126, &"0".repeat(99)).unwrap();
    msg.set_on(127, &"1".repeat(49)).unwrap();

    msg
}

/// Assert ISO-8583 message processing failed with given Response Code
/// and storage has not been altered
async fn assert_noop(
    api: &MockProcessorImpl,
    beneficiary: DevAccount,
    iso_msg: &IsoMsg,
    response_code: ResponseCodes,
    previous_account_state: BankAccount,
    previous_txs: Vec<Transaction>,
) {
    let mut msg_raw = iso_msg.assemble().unwrap();
    let msg_response = api.processor.process(&mut msg_raw).await.unwrap();

    assert_eq!(
        &msg_response.1.bmp_child_value(39).unwrap(),
        Into::<&str>::into(response_code),
    );

    let beneficiary_account = get_bank_account_by_card_number(api, &beneficiary.1).await;

    assert_eq!(beneficiary_account.balance, previous_account_state.balance);

    let beneficiary_txs = api
        .processor
        .transaction_controller
        .find_by_beneficiary(&beneficiary_account.id)
        .await
        .unwrap();

    for (i, tx) in beneficiary_txs.iter().enumerate() {
        assert_eq!(tx, &previous_txs[i]);
    }
}

/// Tests basic payment authorization and its settlement
#[tokio::test]
async fn test_payment_works() {
    let api = MockProcessorImpl::new().await;

    let spec = api.processor.spec.clone();

    let mut new_msg = get_new_iso_msg(&spec, MTI::AuthorizationRequest, ALICE);
    new_msg.set_on(4, "000000000100").unwrap();

    let mut msg_raw = new_msg.assemble().unwrap();
    let msg_response = api.processor.process(&mut msg_raw).await;

    // Assert processing results
    match msg_response {
        Ok((_raw, msg)) => {
            assert_eq!(msg.bmp_child_value(39).unwrap(), "00");
            assert_eq!(msg.bmp_child_value(4).unwrap(), "000000000100");
            assert_eq!(msg.bmp_child_value(126).unwrap(), "0".repeat(99));
            assert_eq!(msg.bmp_child_value(127).unwrap(), "1".repeat(49));
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }

    let alice_account = get_bank_account_by_card_number(&api, &ALICE.1).await;

    let alice_txs = api
        .processor
        .transaction_controller
        .find_by_beneficiary(&alice_account.id)
        .await
        .unwrap();

    assert_eq!(alice_txs.len(), 1);
    let alice_tx = &alice_txs[0];

    assert_eq!(alice_tx.amount, 100);
    assert_eq!(alice_tx.from, alice_account.id);
    assert!(alice_tx.to.is_some());
    assert_eq!(alice_tx.transaction_type, 1); // Credit transaction

    // INSUFFICIENT FUNDS
    // Make sure alice can't spend more than she has
    let mut new_msg = get_new_iso_msg(&spec, MTI::AuthorizationRequest, ALICE);
    new_msg.set_on(4, "000000001000").unwrap();

    assert_noop(
        &api,
        ALICE,
        &new_msg,
        ResponseCodes::InsufficientFunds,
        alice_account.clone(),
        alice_txs.clone(),
    )
    .await;

    // EXPIRED CARD
    // Make sure Eve can't spend anything
    let mut new_msg = get_new_iso_msg(&spec, MTI::AuthorizationRequest, EVE);
    new_msg.set_on(4, "000000000100").unwrap();

    let eve_account = get_bank_account_by_card_number(&api, &EVE.1).await;

    assert_noop(
        &api,
        EVE,
        &new_msg,
        ResponseCodes::ExpiredCard,
        eve_account.clone(),
        vec![],
    )
    .await;

    // INVALID CARD NUMBER
    // Make sure any msg with invalid card number is rejected
    let mut new_msg = get_new_iso_msg(&spec, MTI::AuthorizationRequest, ALICE);
    new_msg.set_on(2, "1234567890123456").unwrap();
    new_msg.set_on(4, "000000000100").unwrap();

    assert_noop(
        &api,
        ALICE,
        &new_msg,
        ResponseCodes::InvalidCardNumber,
        alice_account.clone(),
        alice_txs.clone(),
    )
    .await;

    // INVALID TRANSACTION
    // Make sure any msg with invalid transaction is rejected
    // This is triggered when timestamp is not within 30 seconds of now
    let mut new_msg = get_new_iso_msg(&spec, MTI::AuthorizationRequest, CHARLIE);
    new_msg.set_on(7, "1109010101").unwrap();
    new_msg.set_on(4, "000000000100").unwrap();

    let charlie_account = get_bank_account_by_card_number(&api, &CHARLIE.1).await;

    assert_noop(
        &api,
        CHARLIE,
        &new_msg,
        ResponseCodes::InvalidTransaction,
        charlie_account.clone(),
        vec![],
    )
    .await;

    // DO NOT HONOR
    // Can be caused by wrong cvv
    let mut new_msg = get_new_iso_msg(&spec, MTI::AuthorizationRequest, ALICE);
    new_msg
        .set_on(
            35,
            &format!(
                "{}D{}C{}",
                ALICE.1,
                alice_account
                    .card_expiration_date
                    .format("%m%y")
                    .to_string(),
                "999" // correct cvv is 123
            ),
        )
        .unwrap();
    new_msg.set_on(4, "000000000100").unwrap();

    assert_noop(
        &api,
        ALICE,
        &new_msg,
        ResponseCodes::DoNotHonor,
        alice_account.clone(),
        alice_txs.clone(),
    )
    .await;

    // And now finally, DAVE makes big payment
    let mut new_msg = get_new_iso_msg(&spec, MTI::AuthorizationRequest, DAVE);
    new_msg.set_on(4, "000000100000").unwrap();

    let mut msg_raw = new_msg.assemble().unwrap();
    let msg_response = api.processor.process(&mut msg_raw).await;

    // Assert processing results
    match msg_response {
        Ok((_raw, msg)) => {
            assert_eq!(msg.bmp_child_value(39).unwrap(), "00");
            assert_eq!(msg.bmp_child_value(4).unwrap(), "000000100000");
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }

    let dave_account = get_bank_account_by_card_number(&api, &DAVE.1).await;

    assert_eq!(dave_account.balance, DAVE.3 - 100_000);

    let dave_txs = api
        .processor
        .transaction_controller
        .find_by_beneficiary(&dave_account.id)
        .await
        .unwrap();

    assert_eq!(dave_txs.len(), 1);
    let dave_tx = &dave_txs[0];

    assert_eq!(dave_tx.amount, 100_000);

    // Settlement is `on-us` since merchant is hard coded as `ACQUIRER`
    let acquirer = get_bank_account_by_card_number(&api, &ACQUIRER.1).await;

    assert_eq!(acquirer.balance, ACQUIRER.3 + 100_000 + 100);

    for (alice_tx, dave_tx) in alice_txs.iter().zip(dave_txs.iter()) {
        assert_eq!(alice_tx.to, Some(acquirer.id));
        assert_eq!(dave_tx.to, Some(acquirer.id));
    }
}

/// Tests reversal
#[tokio::test]
async fn test_reversal_works() {
    let api = MockProcessorImpl::new().await;

    let spec = api.processor.spec.clone();

    let mut new_msg = get_new_iso_msg(&spec, MTI::AuthorizationRequest, ALICE);
    new_msg.set("message_type", "0100").unwrap();
}
