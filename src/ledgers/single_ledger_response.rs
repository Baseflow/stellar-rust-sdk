use derive_getters::Getters;
use serde::Deserialize;
use stellar_xdr::curr::{LedgerHeader, ReadXdr, Limits};

use crate::models::Response;

/// Represents the response for a single ledger query from the Stellar Horizon API.
///
/// This struct encapsulates all the details of a single ledger, including its transactions, 
/// operations, and various ledger attributes.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SingleLedgerResponse {
    /// Navigational links related to the ledger.
    _links: Links,
    /// The unique identifier of the ledger.
    id: String,
    /// A token used for paging through results.
    paging_token: String,
    /// The hash of the ledger.
    hash: String,
    /// The hash of the previous ledger.
    prev_hash: String,
    /// The sequence number of the ledger.
    sequence: i32,
    /// The number of successful transactions in the ledger.
    successful_transaction_count: i32,
    /// The number of failed transactions in the ledger.
    failed_transaction_count: i32,
    /// The total number of operations in the ledger.
    operation_count: i32,
    /// The total number of operations in the transaction set.
    tx_set_operation_count: i32,
    /// The closing time of the ledger.
    closed_at: String,
    /// The total number of coins in the ledger.
    total_coins: String,
    /// The total fees in the fee pool.
    fee_pool: String,
    /// The base fee in stroops.
    base_fee_in_stroops: i32,
    /// The base reserve in stroops.
    base_reserve_in_stroops: i64,
    /// The maximum number of transactions in the transaction set.
    max_tx_set_size: i32,
    /// The protocol version used in the ledger.
    protocol_version: i32,
    /// The XDR-encoded header of the ledger.
    header_xdr: String,
}

/// Struct defining navigational links within the single ledger response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Links {
    /// The link to the ledger itself.
    #[serde(rename = "self")]
    self_: Link,
    /// Link to the ledger's transactions.
    transactions: Link,
    /// Link to the ledger's operations.
    operations: Link,
    /// Link to the payments in the ledger.
    payments: Link,
    /// Link to the effects resulting from the ledger.
    effects: Link,
}

/// Represents a single hyperlink within the ledger's response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Link {
    /// The URL of the linked resource.
    href: String,
    /// A boolean indicating if the link is templated.
    templated: Option<bool>,
}

impl Response for SingleLedgerResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

impl SingleLedgerResponse {
    /// Decodes the XDR-encoded header of the ledger.
    pub fn decoded_header_xdr(&self) -> Result<LedgerHeader, String> {
        let encoded = self.header_xdr.as_bytes();
        let decoded = LedgerHeader::from_xdr_base64(encoded, Limits::none()).unwrap();
        Ok(decoded)
    }
}
