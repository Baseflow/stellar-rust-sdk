use crate::models::Response;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct Links {
    #[serde(rename = "self")]
    self_link: Link,
    next: Link,
    prev: Link,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct Link {
    href: Option<String>,
    toml: Option<Toml>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct Toml {
    href: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct Records {
    _links: Link,
    asset_type: String,
    asset_code: String,
    asset_issuer: String,
    paging_token: String,
    num_accounts: u32,
    num_claimable_balances: u32,
    num_liquidity_pools: u32,
    num_contracts: u32,
    amount: String,
    accounts: AccountInfo,
    claimable_balances_amount: String,
    liquidity_pools_amount: String,
    contracts_amount: String,
    balances: AccountBalances,
    flags: Flags,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct AccountInfo {
    authorized: u32,
    authorized_to_maintain_liabilities: u32,
    unauthorized: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct AccountBalances {
    authorized: String,
    authorized_to_maintain_liabilities: String,
    unauthorized: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct Flags {
    auth_required: bool,
    auth_revocable: bool,
    auth_immutable: bool,
    auth_clawback_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct Embedded {
    records: Vec<Records>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct AllAssetsResponse {
    _links: Links,
    _embedded: Embedded,
}

impl Response for AllAssetsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}