use derive_getters::Getters;
use serde::Deserialize;

use crate::models::Response;

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Link {
    #[serde(rename = "self")]
    self_link: SelfLink,
    next: Option<SelfLink>,
    prev: Option<SelfLink>,
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SelfLink {
    href: String,
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Records {
    _links: Link,
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
    base_reserve_in_stroops: i32,
    max_tx_set_size: i32,
    protocol_version: i32,
    header_xdr: String,
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Embedded {
    records: Vec<Records>,
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct LedgersResponse {
    _links: Link,
    _embedded: Embedded,
}

impl Response for LedgersResponse {
    fn from_json(json: String) -> Result<Self, String> {
        // serde_json::from_str(&json).map_err(|e| e.to_string())

        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

// impl LedgersResponse {
//     pub fn get__links(&self) -> Link {
//         self._links.clone()
//     }

//     pub fn get__embedded(&self) -> Embedded {
//         self._embedded.clone()
//     }
// }

// impl Embedded {
//     pub fn get_records(&self) -> Vec<Records> {
//         self.records.clone()
//     }
    
//     pub fn get_single_record(&self, index: usize) -> Records {
//         self.records[index].clone()
//     }
// }

// impl Records {
//     pub fn get__links(&self) -> Link {
//         self._links.clone()
//     }

//     pub fn get_id(&self) -> String {
//         self.id.clone()
//     }

//     pub fn get_paging_token(&self) -> String {
//         self.paging_token.clone()
//     }

//     pub fn get_hash(&self) -> String {
//         self.hash.clone()
//     }

//     pub fn get_prev_hash(&self) -> String {
//         self.prev_hash.clone()
//     }

//     pub fn get_sequence(&self) -> i32 {
//         self.sequence.clone()
//     }

//     pub fn get_successful_transaction_count(&self) -> i32 {
//         self.successful_transaction_count.clone()
//     }

//     pub fn get_failed_transaction_count(&self) -> i32 {
//         self.failed_transaction_count.clone()
//     }

//     pub fn get_operation_count(&self) -> i32 {
//         self.operation_count.clone()
//     }

//     pub fn get_tx_set_operation_count(&self) -> i32 {
//         self.tx_set_operation_count.clone()
//     }

//     pub fn get_closed_at(&self) -> String {
//         self.closed_at.clone()
//     }

//     pub fn get_total_coins(&self) -> String {
//         self.total_coins.clone()
//     }

//     pub fn get_fee_pool(&self) -> String {
//         self.fee_pool.clone()
//     }

//     pub fn get_base_fee_in_stroops(&self) -> i32 {
//         self.base_fee_in_stroops.clone()
//     }

//     pub fn get_base_reserve_in_stroops(&self) -> i32 {
//         self.base_reserve_in_stroops.clone()
//     }

//     pub fn get_max_tx_set_size(&self) -> i32 {
//         self.max_tx_set_size.clone()
//     }

//     pub fn get_protocol_version(&self) -> i32 {
//         self.protocol_version.clone()
//     }

//     pub fn get_header_xdr(&self) -> String {
//         self.header_xdr.clone()
//     }
// }

// impl SelfLink {
//     pub fn get_href(&self) -> String {
//         self.href.clone()
//     }
// }

// impl Link {
//     pub fn get_self_link(&self) -> SelfLink {
//         self.self_link.clone()
//     }

//     pub fn get_next(&self) -> Option<SelfLink> {
//         self.next.clone()
//     }

//     pub fn get_prev(&self) -> Option<SelfLink> {
//         self.prev.clone()
//     }
// }
