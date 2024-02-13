use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::models::Response;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct EffectsForTransactionResponse {
    /// Navigational links for the current, next, and previous pages of the response.
    #[serde(rename = "_links")]
    pub links: Links,
    /// Contains the actual list of effect records in the `records` field.
    #[serde(rename = "_embedded")]
    pub embedded: Embedded,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    /// Navigational links for the current, next, and previous pages of the response.
    #[serde(rename = "self")]
    pub self_field: Link,
    /// Navigational links for the next page of the response.
    pub next: Link,
    /// Navigational links for the previous page of the response.
    pub prev: Link,
}

/// Represents different kinds of links a Response might have.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    /// Navigational links for the current, next, and previous pages of the response.
    #[serde(rename = "self")]
    pub self_field: Option<String>,
    /// Navigational links for the next page of the response.
    pub next: Option<String>,
    /// Navigational links for the previous page of the response.
    pub prev: Option<String>,
    /// Navigational links for the operation of the response.
    pub operation: Option<String>,
    /// Navigational links for the succeeds of the response.
    pub succeeds: Option<String>,
    /// Navigational links for the precedes of the response.
    pub precedes: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    /// A list of effect records.
    pub records: Vec<Record>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    /// Navigational links for the current, next, and previous pages of the response.
    #[serde(rename = "_links")]
    pub links: RecordLinks,
    /// A unique identifier of the effect
    pub id: String,
    #[serde(rename = "paging_token")]
    /// A token used for paging through results.
    pub paging_token: String,
    /// The account that generated the effect.
    pub account: String,
    /// The type of effect.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The type_i of the effect.
    #[serde(rename = "type_i")]
    pub type_i: i64,
    /// the epoch timestamp when the effect was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The starting balance of the effect.
    #[serde(rename = "starting_balance")]
    pub starting_balance: Option<String>,
    /// The asset type of the effect.
    #[serde(rename = "asset_type")]
    pub asset_type: Option<String>,
    /// the amount of the effect
    pub amount: Option<String>,
    /// the wheight of the effect
    pub weight: Option<i64>,
    #[serde(rename = "public_key")]
    /// The public key of the effect
    pub public_key: Option<String>,
    /// The trustor of the effect
    pub key: Option<String>,
}

/// Represents different kinds of links a Record might have.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct RecordLinks {
    /// Navigational links for the current, next, and previous pages of the response.
    pub operation: Link,
    /// Navigational links for the next page of the response.
    pub succeeds: Link,
    /// Navigational links for the previous page of the response.
    pub precedes: Link,
}

impl Response for EffectsForTransactionResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}
