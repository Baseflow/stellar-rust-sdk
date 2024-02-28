use serde::{Deserialize, Serialize};

use crate::models::Response;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllLiquidityPoolsResponse {
    #[serde(rename = "_links")]
    pub links: Links,
    #[serde(rename = "_embedded")]
    pub embedded: Embedded,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: Option<ResponseLink>,
    pub next: Option<ResponseLink>,
    pub prev: Option<ResponseLink>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseLink {
    pub self_link: String,
    pub next: String,
    pub prev: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    pub records: Vec<Record>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(rename = "_links")]
    pub links: RecordLink,
    pub id: String,
    #[serde(rename = "paging_token")]
    pub paging_token: String,
    #[serde(rename = "fee_bp")]
    pub fee_bp: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "total_trustlines")]
    pub total_trustlines: String,
    #[serde(rename = "total_shares")]
    pub total_shares: String,
    pub reserves: Vec<Resef>,
    #[serde(rename = "last_modified_ledger")]
    pub last_modified_ledger: i64,
    #[serde(rename = "last_modified_time")]
    pub last_modified_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordLink {
    #[serde(rename = "self")]
    pub self_field: Option<ResponseLink>,
    pub transactions: Transactions,
    pub operations: Operations,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    pub href: String,
    pub templated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operations {
    pub href: String,
    pub templated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resef {
    pub asset: String,
    pub amount: String,
}

impl Response for AllLiquidityPoolsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}