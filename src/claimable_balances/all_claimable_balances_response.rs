use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::Response;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ClaimableBalancesResponse {
    _links: Links,
    _embedded: EmbeddedRecords,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Links {
    #[serde(rename = "self")]
    self_link: Href,
    next: Href,
    prev: Href,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Href {
    href: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EmbeddedRecords {
    records: Vec<Record>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Record {
    _links: RecordLinks,
    id: String,
    asset: String,
    amount: String,
    sponsor: String,
    last_modified_ledger: u64,
    last_modified_time: String,
    claimants: Vec<Claimant>,
    flags: Flags,
    paging_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RecordLinks {
    #[serde(rename = "self")]
    self_link: Href,
    transactions: Href,
    operations: Href,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Claimant {
    destination: String,
    predicate: Predicate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Predicate {
    Unconditional { unconditional: bool },
    Conditional {
        or: Vec<ConditionalPredicate>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConditionalPredicate {
    #[serde(flatten)]
    inner: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Value {
    String(String),
    Boolean(bool),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Flags {
    clawback_enabled: bool,
}

impl Response for ClaimableBalancesResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

impl ClaimableBalancesResponse {
    pub fn get_links(&self) -> Links {
        self._links.clone()
    }

    pub fn get_embedded(&self) -> EmbeddedRecords {
        self._embedded.clone()
    }
}

impl Links {
    pub fn get_self_link(&self) -> Href {
        self.self_link.clone()
    }

    pub fn get_next(&self) -> Href {
        self.next.clone()
    }

    pub fn get_prev(&self) -> Href {
        self.prev.clone()
    }
}

impl EmbeddedRecords {
    pub fn get_records(&self) -> Vec<Record> {
        self.records.clone()
    }

    pub fn get_single_record(&self, index: usize) -> Record {
        self.records[index].clone()
    }
}

impl Record {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_asset(&self) -> String {
        self.asset.clone()
    }

    pub fn get_amount(&self) -> String {
        self.amount.clone()
    }

    pub fn get_sponsor(&self) -> String {
        self.sponsor.clone()
    }

    pub fn get_last_modified_ledger(&self) -> u64 {
        self.last_modified_ledger
    }

    pub fn get_last_modified_time(&self) -> String {
        self.last_modified_time.clone()
    }

    pub fn get_claimants(&self) -> Vec<Claimant> {
        self.claimants.clone()
    }

    pub fn get_flags(&self) -> Flags {
        self.flags.clone()
    }

    pub fn get_paging_token(&self) -> String {
        self.paging_token.clone()
    }

    pub fn get_links(&self) -> RecordLinks {
        self._links.clone()
    }
}

impl RecordLinks {
    pub fn get_self_link(&self) -> Href {
        self.self_link.clone()
    }

    pub fn get_transactions(&self) -> Href {
        self.transactions.clone()
    }

    pub fn get_operations(&self) -> Href {
        self.operations.clone()
    }
}

impl Claimant {
    pub fn get_destination(&self) -> String {
        self.destination.clone()
    }

    pub fn get_predicate(&self) -> Predicate {
        self.predicate.clone()
    }
}

impl Flags {
    pub fn get_clawback_enabled(&self) -> bool {
        self.clawback_enabled
    }
}

impl Predicate {
    pub fn get_unconditional(&self) -> bool {
        match self {
            Predicate::Unconditional { unconditional } => unconditional.clone(),
            _ => false,
        }
    }

    pub fn get_conditional(&self) -> Vec<ConditionalPredicate> {
        match self {
            Predicate::Conditional { or } => or.clone(),
            _ => Vec::new(),
        }
    }
}

impl ConditionalPredicate {
    pub fn get_inner(&self) -> HashMap<String, Value> {
        self.inner.clone()
    }
}

impl Value {
    pub fn get_string(&self) -> String {
        match self {
            Value::String(string) => string.clone(),
            _ => String::new(),
        }
    }

    pub fn get_boolean(&self) -> bool {
        match self {
            Value::Boolean(boolean) => boolean.clone(),
            _ => false,
        }
    }
}

