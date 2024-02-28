use serde::{Deserialize, Serialize};

use crate::models::Response;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleLiquidityPoolResponse {
    #[serde(rename = "_links")]
    pub links: Links,
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
    pub reserves: Vec<Reserve>,
    #[serde(rename = "last_modified_ledger")]
    pub last_modified_ledger: i64,
    #[serde(rename = "last_modified_time")]
    pub last_modified_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: Self_field,
    pub transactions: Transactions,
    pub operations: Operations,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Self_field {
    pub href: String,
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
pub struct Reserve {
    pub asset: String,
    pub amount: String,
}

impl Response for SingleLiquidityPoolResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}
