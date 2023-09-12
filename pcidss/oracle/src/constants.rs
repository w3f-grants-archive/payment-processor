//! Constants used in the PCIDSS Gateway.

/// Field numbers that we populate in the ISO message
pub const POPULATED_ISO_MSG_FIELD_NUMBERS: [u32; 17] = [
    0,   // Message Type Indicator or MTI
    2,   // Primary account number, card number
    3,   // Processing code
    4, // Amount is 12 characters long, check the length of amount and pad it with `0` from the left
    7, // Transmission date, combination of 13 and 12
    12, // HHMMSS format of transaction time
    13, // MMDD format of transaction date
    14, // Card expiration date
    18, // Merchant Category Code
    32, // Acquiring institution ID
    35, // Track-2 Data
    41, // Card Acceptor Terminal Identification
    42, // Card Acceptor Identification Code
    43, // Card Acceptor Name/Location
    49, // Currency Code, Transaction, USD - 997, EUR - 978
    126, // Private data
    127, // Private data, 100 characters long: this contains transaction hash if the transaction is a reversal
];

/// Response Code field
pub const RESPONSE_CODE_FIELD_NUMBER: u32 = 39;
