use serde::{Deserialize, Serialize};

use crate::models::Response;

/// Represents the response from the Horizon server when querying for all liquidity pools.
///
/// This struct represents the response from the Horizon server when querying for all liquidity pools.
/// It includes the links to the current, next, and previous pages of the response, as well as the
/// embedded records of liquidity pools.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllLiquidityPoolsResponse {
    /// The links to the current, next, and previous pages of the response.
    #[serde(rename = "_links")]
    pub links: Links,
    /// The embedded records of liquidity pools.
    #[serde(rename = "_embedded")]
    pub embedded: Embedded,
}

/// Represents the links in the list of all liquidity pools response.
///
/// This struct includes links such as the self-link (current page), next, and previous,
/// providing quick navigation across different pages of the liquidity pool response.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    /// The link to the current page of the liquidity pool response.
    #[serde(rename = "self")]
    pub self_field: Option<ResponseLink>,
    /// Optional link to the next page of liquidity pool records.
    pub next: Option<ResponseLink>,
    /// Optional link to the previous page of liquidity pool records.
    pub prev: Option<ResponseLink>,
}

/// Represents the navigational links in a liquidity pool response from the Stellar Horizon API.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseLink {
    /// A `String` representing the hyperlink reference to the current resource or query.
    pub href: String,
}

/// Represents the embedded records of liquidity pools in the Horizon API response.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    /// The list of liquidity pool records.
    pub records: Vec<Record>,
}

/// Represents a single liquidity pool record in the Horizon API response.
///
/// This struct encapsulates detailed information about a single liquidity pool, including its ID,
/// fee, type, reserves, and other relevant data.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    /// Navigational links related to the operation of the effect.
    #[serde(rename = "_links")]
    pub links: RecordLink,
    /// The unique identifier of the liquidity pool.
    pub id: String,
    /// A token used for paging through results.
    #[serde(rename = "paging_token")]
    pub paging_token: String,
    /// The fee in basis points.
    #[serde(rename = "fee_bp")]
    pub fee_bp: i64,
    /// The type of the liquidity pool.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The total number of trustlines.
    #[serde(rename = "total_trustlines")]
    pub total_trustlines: String,
    /// The total number of shares.
    #[serde(rename = "total_shares")]
    pub total_shares: String,
    /// The reserves of the liquidity pool.
    pub reserves: Vec<Reserve>,
    /// The last modified ledger.
    #[serde(rename = "last_modified_ledger")]
    pub last_modified_ledger: i64,
    /// The last modified time.
    #[serde(rename = "last_modified_time")]
    pub last_modified_time: String,
}

/// Represents the navigational links belonging to a liquidity pool from the Stellar Horizon API.
///
/// This struct includes links such as the self-link (current liquidity pool), transactions, and operations,
/// providing quick navigation across different pages of the liquidity pool.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordLink {
    /// The link to the current liquidity pool.
    #[serde(rename = "self")]
    pub self_field: Option<ResponseLink>,
    /// The link to the transactions of the liquidity pool.
    pub transactions: Transactions,
    /// The link to the operations of the liquidity pool.
    pub operations: Operations,
}

/// Represents the navigational links in a liquidity pool response from the Stellar Horizon API.
///
/// This struct includes links such as the transactions, and operations,
/// providing quick navigation across different pages of the liquidity pool.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    /// The link to the transactions of the liquidity pool.
    pub href: String,
    /// Optionally indicates if the link is templated
    pub templated: bool,
}

/// Represents the navigational links in a liquidity pool response from the Stellar Horizon API.
///
/// This struct includes links such as the operations,
/// providing quick navigation across different pages of the liquidity pool.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operations {
    /// The link to the operations of the liquidity pool.
    pub href: String,
    /// Optionally indicates if the link is templated
    pub templated: bool,
}

/// Represents a reserve for a liquidity pool. This struct is used to specify the asset code and
/// issuer of the reserve.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reserve {
    /// The asset code of the reserve.
    pub asset: String,
    /// The asset issuer of the reserve.
    pub amount: String,
}

impl Response for AllLiquidityPoolsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}