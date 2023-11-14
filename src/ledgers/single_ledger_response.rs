use derive_getters::Getters;
use serde::{de, Deserialize};
use stellar_xdr::{LedgerHeader, ReadXdr};

use crate::models::Response;

/// Struct defining the self link in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SingleLedgerResponse {
    _links: Links,
    id: String,
    paging_token: String,
    hash: String,
    prev_hash: String,
    sequence: i32,
    successful_transaction_count: i32,
    failed_transaction_count: i32,
    operation_count: i32,
    tx_set_operation_count: i32,
    closed_at: String,
    total_coins: String,
    fee_pool: String,
    base_fee_in_stroops: i32,
    base_reserve_in_stroops: i64,
    max_tx_set_size: i32,
    protocol_version: i32,
    header_xdr: String,
}

/// Struct defining the self link in the list all ledgers response.
#[derive(Debug, Deserialize, Clone, Getters)]
struct Links {
    #[serde(rename = "self")]
    self_: Link,
    transactions: Link,
    operations: Link,
    payments: Link,
    effects: Link,
}

/// Struct defining the self link in the list all ledgers response.
#[derive(Debug, Deserialize, Clone, Getters)]
struct Link {
    href: String,
    templated: Option<bool>,
}

/// Struct defining the embedded records in the list all ledgers response.
impl Response for SingleLedgerResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

/// Struct defining the embedded records in the list all ledgers response.
impl SingleLedgerResponse {
    pub fn decoded_header_xdr(&self) -> Result<LedgerHeader, String> {
        let encoded = self.header_xdr.as_bytes();
        let decoded = stellar_xdr::LedgerHeader::from_xdr_base64(encoded).unwrap();
        Ok(decoded)
    }
}
