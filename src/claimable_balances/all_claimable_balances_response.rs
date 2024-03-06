use super::*;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{models::Response, Embedded, Flags, ResponseLinks};

impl Response for AllClaimableBalancesResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableBalancesResponse {
    #[serde(rename = "_links")]
    pub links: ResponseLinks,
    #[serde(rename = "_embedded")]
    pub embedded: Embedded<ClaimableAssetRecord>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct ClaimableAssetRecord {
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: String,
    pub asset: String,
    pub amount: String,
    pub sponsor: String,
    #[serde(rename = "last_modified_ledger")]
    pub last_modified_ledger: i64,
    #[serde(rename = "last_modified_time")]
    pub last_modified_time: String,
    pub claimants: Vec<Claimant>,
    pub flags: ClaimableBalanceFlag,
    #[serde(rename = "paging_token")]
    pub paging_token: String,
}
