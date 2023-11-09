use serde::{Deserialize, Serialize};

use crate::models::Response;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SelfLink {
    href: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Links {    
    #[serde(rename = "self")]
    self_link: SelfLink,
    next: Option<SelfLink>,
    prev: Option<SelfLink>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Balances {
    balance: String,
    buying_liabilities: String,
    selling_liabilities: String,
    asset_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Thresholds {
    low_threshold: i32,
    med_threshold: i32,
    high_threshold: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Flags {
    auth_required: bool,
    auth_revocable: bool,
    auth_immutable: bool,
    auth_clawback_enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Signers {
    weight: i32,
    key: String,
    #[serde(rename = "type")]
    signer_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Embedded {
    records: Vec<Record>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountsResponse {
    _links: Links,
    _embedded: Embedded,
}

impl Response for AccountsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

impl AccountsResponse {
    pub fn get__links(&self) -> Links {
        self._links.clone()
    }

    pub fn get__embedded(&self) -> Embedded {
        self._embedded.clone()
    }
}

impl Links {
    pub fn get_self_link(&self) -> SelfLink {
        self.self_link.clone()
    }

    pub fn get_next(&self) -> Option<SelfLink> {
        self.next.clone()
    }

    pub fn get_prev(&self) -> Option<SelfLink> {
        self.prev.clone()
    }
}

impl Embedded {
    pub fn get_records(&self) -> Vec<Record> {
        self.records.clone()
    }

    pub fn get_single_record(&self, index: usize) -> Record {
        self.records[index].clone()
    }
}

impl SelfLink {
    pub fn get_href(&self) -> String {
        self.href.clone()
    }
}

impl Record {
    pub fn get__links(&self) -> Links {
        self.links.clone()
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_account_id(&self) -> String {
        self.account_id.clone()
    }

    pub fn get_sequence(&self) -> String {
        self.sequence.clone()
    }

    pub fn get_subentry_count(&self) -> i32 {
        self.subentry_count.clone()
    }

    pub fn get_last_modified_ledger(&self) -> i64 {
        self.last_modified_ledger.clone()
    }

    pub fn get_last_modified_time(&self) -> String {
        self.last_modified_time.clone()
    }

    pub fn get_thresholds(&self) -> Thresholds {
        self.thresholds.clone()
    }

    pub fn get_flags(&self) -> Flags {
        self.flags.clone()
    }

    pub fn get_balances(&self) -> Vec<Balances> {
        self.balances.clone()
    }

    pub fn get_single_balance(&self, index: usize) -> Balances {
        self.balances[index].clone()
    }

    pub fn get_signers(&self) -> Vec<Signers> {
        self.signers.clone()
    }

    pub fn get_single_signer(&self, index: usize) -> Signers {
        self.signers[index].clone()
    }

    pub fn get_data(&self) -> serde_json::Value {
        self.data.clone()
    }

    pub fn get_num_sponsoring(&self) -> i32 {
        self.num_sponsoring.clone()
    }

    pub fn get_num_sponsored(&self) -> i32 {
        self.num_sponsored.clone()
    }

    pub fn get_paging_token(&self) -> String {
        self.paging_token.clone()
    }
}

impl Thresholds {
    pub fn get_low_threshold(&self) -> i32 {
        self.low_threshold.clone()
    }

    pub fn get_med_threshold(&self) -> i32 {
        self.med_threshold.clone()
    }

    pub fn get_high_threshold(&self) -> i32 {
        self.high_threshold.clone()
    }
}

impl Balances {
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

impl Signers {
    pub fn get_weight(&self) -> i32 {
        self.weight.clone()
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_type(&self) -> String {
        self.signer_type.clone()
    }
}