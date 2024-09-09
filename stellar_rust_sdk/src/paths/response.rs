use crate::models::prelude::Embedded;
use crate::models::Response;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// Represents the response for a payment paths query.
///
/// This struct defines the overall structure of the response for a query
/// that retrieves payment paths. It includes the embedded results which
/// consist of the details of each payment path.
///
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct PathsResponse {
    #[serde(rename = "_embedded")]
    embedded: Embedded<Path>,
}

/// Represents a single payment path.
///
/// This struct details a specific payment path including information about
/// the source and destination assets, their amounts, and the sequence of assets
/// that form the path.
///
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct Path {
    /// The type of the source asset.
    source_asset_type: String,
    /// The code of the source asset. Optional.
    source_asset_code: Option<String>,
    /// The issuer of the source asset. Optional.
    source_asset_issuer: Option<String>,
    /// The amount of the source asset.
    source_amount: String,
    /// The type of the destination asset.
    destination_asset_type: String,
    /// The code of the destination asset. Optional.
    destination_asset_code: Option<String>,
    /// The issuer of the destination asset. Optional.
    destination_asset_issuer: Option<String>,
    /// The amount of the destination asset.
    destination_amount: String,
    /// A vector of assets forming the path.
    path: Vec<Asset>,
}

/// Represents a single asset used in the payment path.
///
/// This struct details the information about an asset including its type,
/// code, and issuer.
///
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct Asset {
    asset_type: String,
    asset_code: Option<String>,
    asset_issuer: Option<String>,
}

impl Response for PathsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}
