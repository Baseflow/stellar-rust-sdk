use serde::{de, Deserialize};
use stellar_xdr::{LedgerHeader, ReadXdr};

use crate::models::Response;

#[derive(Debug, Deserialize, Clone)]
pub struct SingleLedgerResponse {
    _links: Links,
    id: String,
    paging_token: String,
    hash: String,
    prev_hash: String,
    sequence: i32,
    successful_transaction_count: i32,
    failed_transaction_count: i32,
    operation_count: i32,
    tx_set_operation_count: i32,
    closed_at: String,
    total_coins: String,
    fee_pool: String,
    base_fee_in_stroops: i32,
    base_reserve_in_stroops: i64,
    max_tx_set_size: i32,
    protocol_version: i32,
    header_xdr: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Links {
    #[serde(rename = "self")]
    self_: Link,
    transactions: Link,
    operations: Link,
    payments: Link,
    effects: Link,
}

#[derive(Debug, Deserialize, Clone)]
struct Link {
    href: String,
    templated: Option<bool>,
}

impl Response for SingleLedgerResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

impl Links {
    pub fn get_self(&self) -> Link {
        self.self_.clone()
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
}

impl Link {
    pub fn get_href(&self) -> String {
        self.href.clone()
    }

    pub fn get_templated(&self) -> bool {
        self.templated.clone().unwrap_or(false)
    }
}

impl SingleLedgerResponse {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_paging_token(&self) -> String {
        self.paging_token.clone()
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_prev_hash(&self) -> String {
        self.prev_hash.clone()
    }

    pub fn get_sequence(&self) -> i32 {
        self.sequence.clone()
    }

    pub fn get_successful_transaction_count(&self) -> i32 {
        self.successful_transaction_count.clone()
    }

    pub fn get_failed_transaction_count(&self) -> i32 {
        self.failed_transaction_count.clone()
    }

    pub fn get_operation_count(&self) -> i32 {
        self.operation_count.clone()
    }

    pub fn get_tx_set_operation_count(&self) -> i32 {
        self.tx_set_operation_count.clone()
    }

    pub fn get_closed_at(&self) -> String {
        self.closed_at.clone()
    }

    pub fn get_total_coins(&self) -> String {
        self.total_coins.clone()
    }

    pub fn get_fee_pool(&self) -> String {
        self.fee_pool.clone()
    }

    pub fn get_base_fee_in_stroops(&self) -> i32 {
        self.base_fee_in_stroops.clone()
    }

    pub fn get_base_reserve_in_stroops(&self) -> i64 {
        self.base_reserve_in_stroops.clone()
    }

    pub fn get_max_tx_set_size(&self) -> i32 {
        self.max_tx_set_size.clone()
    }

    pub fn get_protocol_version(&self) -> i32 {
        self.protocol_version.clone()
    }

    pub fn get_header_xdr(&self) -> String {
        self.header_xdr.clone()
    }

    pub fn get_decoded_header_xdr(&self) -> Result<LedgerHeader, String> {
        let encoded = self.header_xdr.as_bytes();
        let decoded = stellar_xdr::LedgerHeader::from_xdr_base64(encoded).unwrap();
        Ok(decoded)
    }
}
