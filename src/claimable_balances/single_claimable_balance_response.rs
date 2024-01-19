use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
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

/// Contains navigational links related to the single claimable balance response.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    /// The link to the current claimable balance resource.
    #[serde(rename = "self")]
    pub self_field: SelfField,

    /// Link to transactions related to the claimable balance.
    pub transactions: Transactions,

    /// Link to operations related to the claimable balance.
    pub operations: Operations,
}

/// Represents a navigational link in the Horizon API response.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct SelfField {
    /// The URL of the link.
    pub href: String,
}

/// Represents a link to the transactions of a claimable balance.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    /// The URL of the transactions link.
    pub href: String,

    /// Indicates if the link is templated.
    pub templated: bool,
}

/// Represents a link to the operations of a claimable balance.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Operations {
    /// The URL of the operations link.
    pub href: String,

    /// Indicates if the link is templated.
    pub templated: bool,
}

/// Represents a claimant of a claimable balance.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Claimant {
    /// The account ID of the claimant.
    pub destination: String,

    /// Conditions that need to be met for the claimant to claim the balance.
    pub predicate: Predicate,
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

/// Flags indicating special conditions of the claimable balance.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
    /// Indicates if the clawback feature is enabled for the claimable balance.
    #[serde(rename = "clawback_enabled")]
    pub clawback_enabled: bool,
}

impl Response for SingleClaimableBalanceResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}

impl Predicate {
    pub fn is_valid(&self, date: DateTime<Utc>) -> bool {
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
    fn is_valid(&self, date: DateTime<Utc>) -> bool {
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
    fn is_valid(&self, date: DateTime<Utc>) -> bool {
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
    fn is_valid(&self, date: DateTime<Utc>) -> bool {
        date <= parse_epoch(&self.abs_before_epoch)
    }
}

fn parse_epoch(epoch_str: &str) -> DateTime<Utc> {
    // Convert the timestamp string into an i64
    let timestamp = epoch_str.parse::<i64>().unwrap();

    // Create a NaiveDateTime from the timestamp
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();

    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);

    return datetime;
}
