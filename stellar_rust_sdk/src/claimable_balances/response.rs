use super::*;
use crate::models::prelude::*;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

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
    pub embedded: Embedded<ClaimableBalance>,
}

/// Represents a claimable balance query in the Horizon API.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct ClaimableBalance {
    /// Links to related resources in the Horizon API response.
    #[serde(rename = "_links")]
    pub links: ClaimableBalanceLinks,

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
    pub flags: ClaimableBalanceFlag,

    /// A token used for paging through results.
    #[serde(rename = "paging_token")]
    pub paging_token: String,
}

/// Contains navigational links related to the single claimable balance response.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct ClaimableBalanceLinks {
    /// The link to the current claimable balance resource.
    #[serde(rename = "self")]
    pub self_field: Link,

    /// Link to transactions related to the claimable balance.
    pub transactions: TemplateLink,

    /// Link to operations related to the claimable balance.
    pub operations: TemplateLink,
}

/// Represents a claimant of a claimable balance.
#[derive(Default, Debug, Clone, Serialize, PartialEq, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Claimant {
    /// The account ID of the claimant.
    pub destination: String,

    /// Conditions that need to be met for the claimant to claim the balance.
    pub predicate: Predicate,
}

#[derive(Default, Debug, Clone, Serialize, PartialEq, Deserialize, Getters)]
pub struct ClaimableBalanceFlag {
    /// The flag indicating whether the claimable balance is clawback-enabled.
    pub clawback_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Predicate {
    pub unconditional: Option<bool>,
    pub and: Option<Vec<And>>,
    pub or: Option<Vec<Or>>,
    pub not: Option<Not>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct And {
    pub not: Option<Not>,
    #[serde(rename = "abs_before")]
    pub abs_before: Option<String>,
    #[serde(rename = "abs_before_epoch")]
    pub abs_before_epoch: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Not {
    #[serde(rename = "abs_before")]
    pub abs_before: String,
    #[serde(rename = "abs_before_epoch")]
    pub abs_before_epoch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Or {
    #[serde(rename = "abs_before")]
    pub abs_before: Option<String>,
    #[serde(rename = "abs_before_epoch")]
    pub abs_before_epoch: Option<String>,
    pub not: Option<Not>,
}

#[allow(dead_code)]
impl Predicate {
    pub(crate) fn is_valid(&self, date: DateTime<Utc>) -> bool {
        match self {
            Predicate {
                unconditional: Some(true),
                ..
            } => true,
            Predicate {
                and: Some(ands), ..
            } => ands.iter().all(|cond| cond.is_valid(date)),
            Predicate { or: Some(ors), .. } => ors.iter().any(|cond| cond.is_valid(date)),
            Predicate { not: Some(not), .. } => !not.is_valid(date),
            _ => false,
        }
    }
}

impl And {
    pub(crate) fn is_valid(&self, date: DateTime<Utc>) -> bool {
        if let Some(not) = &self.not {
            if not.is_valid(date) {
                return false;
            }
        }
        self.abs_before_epoch
            .as_ref()
            .map(|d| date < parse_epoch(d))
            .unwrap_or(true)
    }
}

impl Or {
    pub(crate) fn is_valid(&self, date: DateTime<Utc>) -> bool {
        if let Some(not) = &self.not {
            if not.is_valid(date) {
                return true;
            }
        }
        self.abs_before_epoch
            .as_ref()
            .map(|d| date < parse_epoch(d))
            .unwrap_or(false)
    }
}

impl Not {
    pub(crate) fn is_valid(&self, date: DateTime<Utc>) -> bool {
        date <= parse_epoch(&self.abs_before_epoch)
    }
}

impl Response for ClaimableBalance {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;
        Ok(response)
    }
}
