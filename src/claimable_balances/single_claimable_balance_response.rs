use chrono::DateTime;
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

/// Represents conditions under which a claimable balance can be claimed.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Predicate {
    /// Indicates if the claim is unconditional.
    pub unconditional: Option<bool>,

    /// Conditions combined using the logical 'or' operation.
    pub or: Option<Vec<Or>>,
}

/// Represents an 'or' condition in a claimable balance's claim predicate.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Or {
    /// Conditions combined using the logical 'and' operation.
    pub and: Option<Vec<And>>,

    /// Specifies the absolute time before which the claim is valid.
    #[serde(rename = "abs_before")]
    pub abs_before: Option<String>,

    /// The epoch time representation of `abs_before`.
    #[serde(rename = "abs_before_epoch")]
    pub abs_before_epoch: Option<String>,
}

/// Represents an 'and' condition in a claimable balance's claim predicate.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct And {
    /// A negation of a condition.
    pub not: Option<Not>,

    /// Specifies the absolute time before which the claim is valid.
    #[serde(rename = "abs_before")]
    pub abs_before: Option<String>,

    /// The epoch time representation of `abs_before`.
    #[serde(rename = "abs_before_epoch")]
    pub abs_before_epoch: Option<String>,
}

/// Represents a 'not' condition in a claimable balance's claim predicate.
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Not {
    /// Specifies the absolute time before which the claim is invalid.
    #[serde(rename = "abs_before")]
    pub abs_before: String,

    /// The epoch time representation of `abs_before`.
    #[serde(rename = "abs_before_epoch")]
    pub abs_before_epoch: String,
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

/// This method checks if a claim is valid at a specific datetime.
impl Predicate {
    // This method checks if a claim is valid at a specific datetime.
    pub fn is_valid_claim(&self, datetime: DateTime<Utc>) -> bool {
        // If the predicate is marked as unconditional, the claim is always valid.
        if let Some(true) = self.unconditional {
            true
        } 
        // If there are 'or' conditions, check if any of these conditions validate the claim.
        else if let Some(or_conditions) = &self.or {
            or_conditions.iter().any(|or| or.is_valid(datetime))
        } 
        // If there are no conditions, the claim is valid.
        else {
            true
        }
    }
}


impl Or {
    // This method checks if any condition under 'or' validates the claim.
    fn is_valid(&self, datetime: DateTime<Utc>) -> bool {
        // If there are 'and' conditions, check if any combination of these conditions is valid.
        if let Some(and_conditions) = &self.and {
            and_conditions.iter().any(|and| and.is_valid(datetime))
        } 
        // If there is an 'abs_before' condition, check if the datetime is before this date.
        else if let Some(abs_before) = &self.abs_before {
            if let Ok(abs_before_date) = DateTime::parse_from_rfc3339(abs_before) {
                datetime < abs_before_date
            } else {
                false
            }
        } 
        // If no specific condition is found, the claim is valid.
        else {
            true
        }
    }
}

impl And {
    // This method checks if all conditions under 'and' are met.
    fn is_valid(&self, datetime: DateTime<Utc>) -> bool {
        let mut is_valid = true;

        // If there is an 'abs_before' condition, check if the datetime is before this date.
        if let Some(abs_before) = &self.abs_before {
            if let Ok(abs_before_date) = DateTime::parse_from_rfc3339(abs_before) {
                is_valid &= datetime < abs_before_date;
            }
        }

        // If there is a 'not' condition, it should also validate the datetime.
        if let Some(not_condition) = &self.not {
            is_valid &= not_condition.is_valid(datetime);
        }

        is_valid
    }
}

impl Not {
    // This method checks if the datetime does not fall before the specified date, negating the condition.
    fn is_valid(&self, datetime: DateTime<Utc>) -> bool {
        if let Ok(not_before_date) = DateTime::parse_from_rfc3339(&self.abs_before) {
            datetime >= not_before_date
        } else {
            false
        }
    }
}

