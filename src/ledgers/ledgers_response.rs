use derive_getters::Getters;
use serde::Deserialize;

use crate::models::Response;

/// Represents the navigational links in a ledger response from the Stellar Horizon API.
///
/// This struct includes links such as the self-link (current page), next, and previous, 
/// providing quick navigation across different pages of the ledger response.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct LedgersResponseLink {
    /// The link to the current page of the ledger response.
    #[serde(rename = "self")]
    self_link: SelfLink,
    /// Optional link to the next page of ledger records.
    next: Option<SelfLink>,
    /// Optional link to the previous page of ledger records.
    prev: Option<SelfLink>,
}

/// Represents a self-link in the ledger response.
///
/// This struct defines the structure of the self-link (`href`) found in the ledgers response
/// from the Horizon API. It contains the URL to the current resource or query.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SelfLink {
    /// A `String` representing the hyperlink reference to the current resource or query.
    href: String,
}

/// Represents a single ledger record in the Horizon API response.
///
/// This struct encapsulates detailed information about a single ledger, including its ID,
/// transaction counts, operation counts, timestamps, and other relevant data.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Records {
    /// Navigational links related to the ledger.
    _links: LedgersResponseLink,
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

/// Contains the embedded ledger records in the all ledgers response.
///
/// This struct encapsulates a collection of ledger records (`Records`) returned in the response.
/// It provides a way to access each individual ledger record and its detailed information.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Embedded {
    /// A vector of individual ledger records.
    records: Vec<Records>,
}

/// Represents the response to a request for listing all ledgers from the Stellar Horizon API.
///
/// This struct contains the overall structure of the response for querying all ledgers. It includes
/// navigational links and a collection of ledger records, each with comprehensive details about the ledger.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct LedgersResponse {
    /// Navigational links for the current, next, and previous pages of the response.
    _links: LedgersResponseLink,
    /// Contains the actual list of ledger records in the `records` field.
    #[serde(rename = "_embedded")]
    embedded: Embedded,
}

impl Response for LedgersResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}
