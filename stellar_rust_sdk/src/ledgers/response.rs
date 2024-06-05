use derive_getters::Getters;
use serde::Deserialize;
use stellar_xdr::curr::{LedgerHeader, Limits, ReadXdr};

use crate::models::prelude::*;

/// Represents the navigational links in a single ledger response from the Horizon API.
///
/// This struct includes various hyperlinks such as links to the ledger itself,
/// transactions, operations, payments and effects
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct LedgerLinks {
    #[serde(rename = "self")]
    pub self_link: Link,
    pub transactions: TemplateLink,
    pub operations: TemplateLink,
    pub payments: TemplateLink,
    pub effects: TemplateLink,
}

/// Represents the response for a single ledger query in the Horizon API.
///
/// This struct defines the overall structure of the response for a ledger offer query.
/// It includes navigational links, offer identifiers, the ID, the hash, the sequence number,
/// and additional data
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Ledger {
    /// Navigational links related to the ledger.
    #[serde(rename = "_links")]
    links: LedgerLinks,
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
    /// The number of operations in the transaction set.
    tx_set_operation_count: i32,
    /// The closing time of the ledger.
    closed_at: String,
    /// The total number of coins in the ledger.
    total_coins: String,
    /// The total fees in the ledger's fee pool.
    fee_pool: String,
    /// The base fee in stroops for transactions in the ledger.
    base_fee_in_stroops: i32,
    /// The base reserve in stroops required for an account in the ledger.
    base_reserve_in_stroops: i32,
    /// The maximum size of a transaction set in the ledger.
    max_tx_set_size: i32,
    /// The protocol version used in the ledger.
    protocol_version: i32,
    /// The XDR-encoded header of the ledger.
    header_xdr: String,
}

/// Represents the response to a request for listing all ledgers from the Stellar Horizon API.
///
/// This struct contains the overall structure of the response for querying all ledgers. It includes
/// navigational links and a collection of ledger records, each with comprehensive details about the ledger.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct LedgersResponse {
    /// Navigational links for the current, next, and previous pages of the response.
    #[serde(rename = "_links")]
    links: ResponseLinks,
    /// Contains the actual list of ledger records in the `records` field.
    #[serde(rename = "_embedded")]
    embedded: Embedded<Ledger>,
}

impl Response for LedgersResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

impl Response for Ledger {
    fn from_json(json: String) -> Result<Self, String> {
        let ledger_record = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(ledger_record)
    }
}

impl Ledger {
    /// Decodes the XDR-encoded header of the ledger.
    pub fn decoded_header_xdr(&self) -> Result<LedgerHeader, String> {
        let encoded = self.header_xdr.as_bytes();
        let decoded = LedgerHeader::from_xdr_base64(encoded, Limits::none()).unwrap();
        Ok(decoded)
    }
}
