use crate::models::prelude::*;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

// TODO: Documentation
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Links {
    #[serde(rename = "self")]
    self_link: Link,
    account: Link,
    ledger: Link,
    operations: Link,
    effects: Link,
    precedes: Link,
    succeeds: Link,
    transaction: Link,
}

// TODO: Documentation
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
pub struct Preconditions {
    pub timebounds: Timebounds,
}

// TODO: Documentation
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
pub struct Timebounds {
    pub min_time: String,
    pub max_time: Option<String>,
}

// TODO: Documentation
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct TransactionResponse {
        #[serde(rename = "_links")]
        pub links: Links,
        pub id: String,
        pub paging_token: String,
        pub successful: bool,
        pub hash: String,
        pub ledger: i64,
        pub created_at: String,
        pub source_account: String,
        pub source_account_sequence: String,
        pub fee_account: String,
        pub fee_charged: String,
        pub max_fee: String,
        pub operation_count: i64,
        pub envelope_xdr: String,
        pub result_xdr: String,
        pub result_meta_xdr: String,
        pub fee_meta_xdr: String,
        pub memo_type: String,
        pub signatures: Vec<String>,
        pub valid_after: String,
        pub valid_before: String,
        pub preconditions: Preconditions,
}

impl Response for TransactionResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}