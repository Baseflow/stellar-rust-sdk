extern crate serde;
extern crate serde_json;

use serde::Deserialize;

use crate::models::Response;

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
pub struct SelfLink {
    href: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Link {
    href: String,
    templated: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Thresholds {
    low_threshold: u32,
    med_threshold: u32,
    high_threshold: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Flags {
    auth_required: bool,
    auth_revocable: bool,
    auth_immutable: bool,
    auth_clawback_enabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Balance {
    balance: String,
    buying_liabilities: String,
    selling_liabilities: String,
    asset_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Signer {
    weight: u32,
    key: String,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Data {}

#[derive(Debug, Deserialize, Clone)]
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

impl SelfLink {
    pub fn get_href(&self) -> String {
        self.href.clone()
    }
}

impl Link {
    pub fn get_href(&self) -> String {
        self.href.clone()
    }

    pub fn get_templated(&self) -> bool {
        self.templated.clone()
    }
}

impl Links {
    pub fn get_self_link(&self) -> SelfLink {
        self.self_link.clone()
    }

    pub fn get_transactions(&self) -> Link {
        self.transactions.clone()
    }

    pub fn get_operations(&self) -> Link {
        self.operations.clone()
    }

    pub fn get_payments(&self) -> Link {
        self.payments.clone()
    }

    pub fn get_effects(&self) -> Link {
        self.effects.clone()
    }

    pub fn get_offers(&self) -> Link {
        self.offers.clone()
    }

    pub fn get_trades(&self) -> Link {
        self.trades.clone()
    }

    pub fn get_data(&self) -> Link {
        self.data.clone()
    }
}

impl Thresholds {
    pub fn get_low_threshold(&self) -> u32 {
        self.low_threshold.clone()
    }

    pub fn get_med_threshold(&self) -> u32 {
        self.med_threshold.clone()
    }

    pub fn get_high_threshold(&self) -> u32 {
        self.high_threshold.clone()
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

impl Balance {
    pub fn get_balance(&self) -> String {
        self.balance.clone()
    }

    pub fn get_buying_liabilities(&self) -> String {
        self.buying_liabilities.clone()
    }

    pub fn get_selling_liabilities(&self) -> String {
        self.selling_liabilities.clone()
    }

    pub fn get_asset_type(&self) -> String {
        self.asset_type.clone()
    }
}

impl Signer {
    pub fn get_weight(&self) -> u32 {
        self.weight.clone()
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_type(&self) -> String {
        self.type_.clone()
    }
}

impl Data {
    pub fn get_value(&self) -> String {
        "".to_string()
    }
}

impl SingleAccountsResponse {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_account_id(&self) -> String {
        self.account_id.clone()
    }

    pub fn get_sequence(&self) -> String {
        self.sequence.clone()
    }

    pub fn get_subentry_count(&self) -> u32 {
        self.subentry_count.clone()
    }

    pub fn get_last_modified_ledger(&self) -> u64 {
        self.last_modified_ledger.clone()
    }

    pub fn get_last_modified_time(&self) -> String {
        self.last_modified_time.clone()
    }

    pub fn get_thresholds(&self) -> Thresholds {
        self.thresholds.clone()
    }

    pub fn get_balances(&self) -> Vec<Balance> {
        self.balances.clone()
    }

    pub fn get_signers(&self) -> Vec<Signer> {
        self.signers.clone()
    }

    pub fn get_data(&self) -> Option<Data> {
        self.data.clone()
    }

    pub fn get_num_sponsoring(&self) -> u32 {
        self.num_sponsoring.clone()
    }

    pub fn get_num_sponsored(&self) -> u32 {
        self.num_sponsored.clone()
    }

    pub fn get_paging_token(&self) -> String {
        self.paging_token.clone()
    }

    pub fn get_flags(&self) -> Flags {
        self.flags.clone()
    }
}