use super::*;
use derive_getters::Getters;
use serde::Deserialize;
use serde::Serialize;

use crate::models::Response;

/// Represents the response to a single claimable balance query in the Horizon API.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct SingleClaimableBalanceResponse {
    /// Links to related resources in the Horizon API response.
    #[serde(rename = "_links")]
    pub links: Links,

    /// The unique identifier of the claimable balance.
    pub id: String,

    /// The asset type of the claimable balance.
    pub asset: String,

    /// The amount of the claimable balance.
    pub amount: String,

    /// The account ID of the sponsor of the claimable balance.
    pub sponsor: String,

    /// The ledger number in which the claimable balance was last modified.
    #[serde(rename = "last_modified_ledger")]
    pub last_modified_ledger: i64,

    /// The timestamp when the claimable balance was last modified.
    #[serde(rename = "last_modified_time")]
    pub last_modified_time: String,

    /// A list of claimants eligible to claim the balance.
    pub claimants: Vec<Claimant>,

    /// Flags indicating special conditions of the claimable balance.
    pub flags: Flags,

    /// A token used for paging through results.
    #[serde(rename = "paging_token")]
    pub paging_token: String,
}

impl Response for SingleClaimableBalanceResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}
