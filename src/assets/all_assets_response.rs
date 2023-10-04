use crate::models::Response;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Links {
    #[serde(rename = "self")]
    self_link: Link,
    next: Link,
    prev: Link,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Link {
    href: Option<String>,
    toml: Option<Toml>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Toml {
    href: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountInfo {
    authorized: u32,
    authorized_to_maintain_liabilities: u32,
    unauthorized: u32,
}

#[derive(Debug, Serialize, Deserialize, Getters, Clone)]
pub struct AccountBalances {
    authorized: String,
    authorized_to_maintain_liabilities: String,
    unauthorized: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flags {
    auth_required: bool,
    auth_revocable: bool,
    auth_immutable: bool,
    auth_clawback_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Embedded {
    records: Vec<Records>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl AllAssetsResponse {
    pub fn get__links(&self) -> Links {
        self._links.clone()
    }

    pub fn get__embedded(&self) -> Embedded {
        self._embedded.clone()
    }
}

impl Records {
    pub fn get_asset_type(&self) -> String {
        self.asset_type.clone()
    }

    pub fn get_asset_code(&self) -> String {
        self.asset_code.clone()
    }

    pub fn get_asset_issuer(&self) -> String {
        self.asset_issuer.clone()
    }

    pub fn get_paging_token(&self) -> String {
        self.paging_token.clone()
    }

    pub fn get_num_accounts(&self) -> u32 {
        self.num_accounts.clone()
    }

    pub fn get_num_claimable_balances(&self) -> u32 {
        self.num_claimable_balances.clone()
    }

    pub fn get_num_liquidity_pools(&self) -> u32 {
        self.num_liquidity_pools.clone()
    }

    pub fn get_num_contracts(&self) -> u32 {
        self.num_contracts.clone()
    }

    pub fn get_amount(&self) -> String {
        self.amount.clone()
    }

    pub fn get_accounts(&self) -> AccountInfo {
        self.accounts.clone()
    }

    pub fn get_claimable_balances_amount(&self) -> String {
        self.claimable_balances_amount.clone()
    }

    pub fn get_liquidity_pools_amount(&self) -> String {
        self.liquidity_pools_amount.clone()
    }

    pub fn get_contracts_amount(&self) -> String {
        self.contracts_amount.clone()
    }

    pub fn get_balances(&self) -> AccountBalances {
        self.balances.clone()
    }

    pub fn get_flags(&self) -> Flags {
        self.flags.clone()
    }
}

impl Embedded {
    pub fn get_records(&self) -> Vec<Records> {
        self.records.clone()
    }

    pub fn get_single_record(&self, index: usize) -> Records {
        self.records[index].clone()
    }
}

impl Links {
    pub fn get_self_link(&self) -> Link {
        self.self_link.clone()
    }

    pub fn get_next_link(&self) -> Link {
        self.next.clone()
    }

    pub fn get_prev_link(&self) -> Link {
        self.prev.clone()
    }
}

impl Link {
    pub fn get_href(&self) -> Option<String> {
        self.href.clone()
    }

    pub fn get_toml(&self) -> Option<Toml> {
        self.toml.clone()
    }
}

impl Toml {
    pub fn get_href(&self) -> String {
        self.href.clone()
    }
}

impl AccountInfo {
    pub fn get_authorized(&self) -> u32 {
        self.authorized.clone()
    }

    pub fn get_authorized_to_maintain_liabilities(&self) -> u32 {
        self.authorized_to_maintain_liabilities.clone()
    }

    pub fn get_unauthorized(&self) -> u32 {
        self.unauthorized.clone()
    }
}

impl AccountBalances {
    pub fn get_authorized(&self) -> String {
        self.authorized.clone()
    }

    pub fn get_authorized_to_maintain_liabilities(&self) -> String {
        self.authorized_to_maintain_liabilities.clone()
    }

    pub fn get_unauthorized(&self) -> String {
        self.unauthorized.clone()
    }
}

impl Flags {
    pub fn get_auth_required(&self) -> bool {
        self.auth_required.clone()
    }

    pub fn get_auth_revocable(&self) -> bool {
        self.auth_revocable.clone()
    }

    pub fn get_auth_immutable(&self) -> bool {
        self.auth_immutable.clone()
    }

    pub fn get_auth_clawback_enabled(&self) -> bool {
        self.auth_clawback_enabled.clone()
    }
}
