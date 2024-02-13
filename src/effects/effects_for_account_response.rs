use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::models::Response;

/// Represents the response to a request for listing all effects for account from the Stellar Horizon API.
///
/// This struct contains the overall structure of the response for querying all effects fir accounts. It includes
/// navigational links and a collection of effects for account records, each with comprehensive details about the effect.
///
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct EffectsForAccountResponse {
    /// Navigational links for the current, next, and previous pages of the response.
    #[serde(rename = "_links")]
    pub links: Links,
    /// Contains the actual list of effect records in the `records` field.
    #[serde(rename = "_embedded")]
    pub embedded: Embedded,
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

/// Represents the links of the whole response
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

/// Represents a collection of effect records in the response.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    /// A list of effect records.
    pub records: Vec<Record>,
}

/// Represents an individual effect record in the response.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    /// Navigational links for the current, next, and previous pages of the response.
    #[serde(rename = "_links")]
    pub links: RecordLinks,
    /// A unique identifier of the effect
    pub id: String,
    /// A token used for paging through results.
    #[serde(rename = "paging_token")]
    pub paging_token: String,
    /// The account that generated the effect.
    pub account: String,
    /// The type of the effect.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The asset code of the effect.
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
    /// The public key of the effect
    #[serde(rename = "public_key")]
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

impl Response for EffectsForAccountResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}
