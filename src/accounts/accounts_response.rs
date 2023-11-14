use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::models::Response;

#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct SelfLink {
    href: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Links {    
    #[serde(rename = "self")]
    self_link: SelfLink,
    next: Option<SelfLink>,
    prev: Option<SelfLink>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Balances {
    balance: String,
    buying_liabilities: String,
    selling_liabilities: String,
    asset_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Thresholds {
    low_threshold: i32,
    med_threshold: i32,
    high_threshold: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Flags {
    auth_required: bool,
    auth_revocable: bool,
    auth_immutable: bool,
    auth_clawback_enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Signers {
    weight: i32,
    key: String,
    #[serde(rename = "type")]
    signer_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Record {
    #[serde(rename = "_links")]
    links: Links,
    id: String,
    account_id: String,
    sequence: String,
    subentry_count: i32,
    last_modified_ledger: i64,
    last_modified_time: String,
    thresholds: Thresholds,
    flags: Flags,
    balances: Vec<Balances>,
    signers: Vec<Signers>,
    data: serde_json::Value,
    num_sponsoring: i32,
    num_sponsored: i32,
    paging_token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Embedded {
    records: Vec<Record>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct AccountsResponse {
    _links: Links,
    _embedded: Embedded,
}

impl Response for AccountsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}