use super::*;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::models::Response;

impl Response for AllClaimableBalancesResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableBalancesResponse {
    #[serde(rename = "_links")]
    pub links: NavigationLinks,
    #[serde(rename = "_embedded")]
    pub embedded: Embedded,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    pub records: Vec<Record>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Record {
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
    pub flags: Flags,
    #[serde(rename = "paging_token")]
    pub paging_token: String,
}
