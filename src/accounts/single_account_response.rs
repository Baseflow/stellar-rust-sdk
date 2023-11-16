extern crate serde;
extern crate serde_json;

use derive_getters::Getters;
use serde::Deserialize;

use crate::models::Response;

/// Struct defining the self link in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Links {
    #[serde(rename = "self")]
    self_link: SelfLink,
    transactions: Link,
    operations: Link,
    payments: Link,
    effects: Link,
    offers: Link,
    trades: Link,
    data: Link,
}

/// Struct defining the self link in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SelfLink {
    href: String,
}

/// Struct defining a single balance in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Link {
    href: String,
    templated: bool,
}

/// Struct defining the thresholds in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Thresholds {
    low_threshold: u32,
    med_threshold: u32,
    high_threshold: u32,
}

/// Struct defining the flags in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Flags {
    auth_required: bool,
    auth_revocable: bool,
    auth_immutable: bool,
    auth_clawback_enabled: bool,
}

/// Struct defining a single signer in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Balance {
    balance: String,
    buying_liabilities: String,
    selling_liabilities: String,
    asset_type: String,
}

/// Struct defining a single signer in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Signer {
    weight: u32,
    key: String,
    #[serde(rename = "type")]
    type_: String,
}

/// Struct defining a record of a single account in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Data {}

/// Struct defining a record of a single account in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SingleAccountsResponse {
    _links: Links,
    id: String,
    account_id: String,
    sequence: String,
    subentry_count: u32,
    last_modified_ledger: u64,
    last_modified_time: String,
    thresholds: Thresholds,
    flags: Flags,
    balances: Vec<Balance>,
    signers: Vec<Signer>,
    data: Option<Data>,
    num_sponsoring: u32,
    num_sponsored: u32,
    paging_token: String,
}

impl Response for SingleAccountsResponse {
    fn from_json(json: String) -> Result<Self, std::io::Error> {
        serde_json::from_str(&json).map_err(|e| 
            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        )
    }
}