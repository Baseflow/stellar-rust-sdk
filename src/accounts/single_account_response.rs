extern crate serde;
extern crate serde_json;

use derive_getters::Getters;
use serde::Deserialize;

use crate::models::Response;

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

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SelfLink {
    href: String,
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Link {
    href: String,
    templated: bool,
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Thresholds {
    low_threshold: u32,
    med_threshold: u32,
    high_threshold: u32,
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Flags {
    auth_required: bool,
    auth_revocable: bool,
    auth_immutable: bool,
    auth_clawback_enabled: bool,
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Balance {
    balance: String,
    buying_liabilities: String,
    selling_liabilities: String,
    asset_type: String,
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Signer {
    weight: u32,
    key: String,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Data {}

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
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}