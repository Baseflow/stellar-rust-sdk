use derive_getters::Getters;
use serde::Deserialize;

use crate::models::Response;

/// Struct defining the self link in the list all ledgers response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Link {
    #[serde(rename = "self")]
    self_link: SelfLink,
    next: Option<SelfLink>,
    prev: Option<SelfLink>,
}

/// Struct defining the self link in the list all ledgers response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SelfLink {
    href: String,
}

/// Struct defining a record of a single ledger in the list all ledgers response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Records {
    _links: Link,
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
    base_reserve_in_stroops: i32,
    max_tx_set_size: i32,
    protocol_version: i32,
    header_xdr: String,
}

/// Struct defining the embedded records in the list all ledgers response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Embedded {
    records: Vec<Records>,
}

/// Struct defining the list all ledgers response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct LedgersResponse {
    _links: Link,
    _embedded: Embedded,
}

impl Response for LedgersResponse {
    fn from_json(json: String) -> Result<Self, std::io::Error> {
        // serde_json::from_str(&json).map_err(|e| e.to_string())

        serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }
}
