use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::models::Response;

/// Struct defining the self link in the list all accounts response.
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct AccountsResponseSelfLink {
    href: String,
}

/// Struct defining the links in the list all accounts response.
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct AccountsResponseLinks {    
    #[serde(rename = "self")]
    self_link: AccountsResponseSelfLink,
    next: Option<AccountsResponseSelfLink>,
    prev: Option<AccountsResponseSelfLink>,
}

/// Struct defining a single balance in the list all accounts response.
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Balances {
    balance: String,
    buying_liabilities: String,
    selling_liabilities: String,
    asset_type: String,
}

/// Struct defining the thresholds in the list all accounts response.
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct AccountsResponseThresholds {
    low_threshold: i32,
    med_threshold: i32,
    high_threshold: i32,
}

/// Struct defining the flags in the list all accounts response.
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct AccountsResponseFlags {
    auth_required: bool,
    auth_revocable: bool,
    auth_immutable: bool,
    auth_clawback_enabled: bool,
}

/// Struct defining a single signer in the list all accounts response.
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Signers {
    weight: i32,
    key: String,
    #[serde(rename = "type")]
    signer_type: String,
}

/// Struct defining a record of a single account in the list all accounts response.
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Record {
    #[serde(rename = "_links")]
    links: AccountsResponseLinks,
    id: String,
    account_id: String,
    sequence: String,
    subentry_count: i32,
    last_modified_ledger: u64,
    last_modified_time: String,
    thresholds: AccountsResponseThresholds,
    flags: AccountsResponseFlags,
    balances: Vec<Balances>,
    signers: Vec<Signers>,
    data: serde_json::Value,
    num_sponsoring: i32,
    num_sponsored: i32,
    paging_token: String,
}

/// Struct defining the embedded object in the list all accounts response.
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Embedded {
    records: Vec<Record>,
}

/// Struct defining the list all accounts response.
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct AccountsResponse {
    _links: AccountsResponseLinks,
    _embedded: Embedded,
}

impl Response for AccountsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}