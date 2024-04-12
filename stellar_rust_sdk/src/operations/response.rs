use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::models::{
    prelude::{Embedded, Link, ResponseLinks},
    Response,
};

#[derive(Serialize, Deserialize, Getters, Debug)]
pub struct OperationResponse {
    #[serde(rename = "_links")]
    pub links: ResponseLinks,
    #[serde(rename = "_embedded")]
    pub embedded: Embedded<Operation>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    #[serde(rename = "_links")]
    pub links: OperationLinks,
    pub id: String,
    #[serde(rename = "paging_token")]
    pub paging_token: String,
    #[serde(rename = "transaction_successful")]
    pub transaction_successful: bool,
    #[serde(rename = "source_account")]
    pub source_account: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "type_i")]
    pub type_i: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "transaction_hash")]
    pub transaction_hash: String,
    #[serde(rename = "starting_balance")]
    pub starting_balance: String,
    pub funder: String,
    pub account: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct OperationLinks {
    #[serde(rename = "self")]
    pub self_field: Link,
    pub transaction: Link,
    pub effects: Link,
    pub succeeds: Link,
    pub precedes: Link,
}

impl Response for OperationResponse {
    fn from_json(json: String) -> Result<Self, String> {
        println!("json: {}", json);

        let operation_record = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(operation_record)
    }
}

impl Response for Operation {
    fn from_json(json: String) -> Result<Self, String> {

        let operation_record = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(operation_record)
    }
}
