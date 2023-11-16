use crate::models::Response;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// Struct defining the self link in the list all accounts response.
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct Links {
    #[serde(rename = "self")]
    self_link: Link,
    next: Link,
    prev: Link,
}

/// Struct defining the self link in the list all accounts response.
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct Link {
    href: Option<String>,
    toml: Option<Toml>,
}

/// Struct defining the self link in the list all accounts response.
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct Toml {
    href: String,
}

/// Struct defining a single balance in the list all accounts response.
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

/// Struct defining a single balance in the list all accounts response.
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct AccountInfo {
    authorized: u32,
    authorized_to_maintain_liabilities: u32,
    unauthorized: u32,
}

/// Struct defining a single balance in the list all accounts response.
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct AccountBalances {
    authorized: String,
    authorized_to_maintain_liabilities: String,
    unauthorized: String,
}

/// Struct defining the thresholds in the list all accounts response.
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct Flags {
    auth_required: bool,
    auth_revocable: bool,
    auth_immutable: bool,
    auth_clawback_enabled: bool,
}

/// Struct defining the embedded in the list all accounts response.
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct Embedded {
    records: Vec<Records>,
}

/// Struct defining the list all accounts response.
#[derive(Debug, Serialize, Deserialize, Clone, Getters)]
pub struct AllAssetsResponse {
    _links: Links,
    _embedded: Embedded,
}

impl Response for AllAssetsResponse {
    fn from_json(json: String) -> Result<Self, std::io::Error> {
        let response = serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        Ok(response)
    }
}
