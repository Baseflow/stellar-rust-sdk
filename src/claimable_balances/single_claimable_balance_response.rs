use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use derive_getters::Getters;
use serde::Deserialize;
use serde::Serialize;

use crate::models::Response;

/// Represents the response to a single claimable balance query in the Horizon API.
///
/// This struct contains detailed information about a single claimable balance, including its ID,
/// asset type, amount, sponsor, and other relevant data.
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

/// Represents a predicate in the claimable balance record.
///
/// This struct defines the structure of a predicate found in the claimable balance record
/// from the Horizon API. It contains the unconditional, and, or, and not fields of the predicate.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Predicate {
    /// A `bool` indicating whether the predicate is unconditional.
    pub unconditional: Option<bool>,
    /// A list of AND conditions that need to be met for the claimant to claim the balance.
    pub and: Option<Vec<And>>,
    /// A list of OR conditions that need to be met for the claimant to claim the balance.
    pub or: Option<Vec<Or>>,
    /// A NOT condition that needs to be met for the claimant to claim the balance.
    pub not: Option<Not>,
}

/// Represents an and predicate in the claimable balance record.
///
/// This struct defines the structure of an and predicate found in the claimable balance record
/// from the Horizon API. It contains the not, abs_before, and abs_before_epoch fields of the and predicate.
///
/// The And predicate defines a set of date time conditions of which all have to be true in order for the given
/// date time to be valid
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct And {
    /// A not predicate. A And predicate can have a Not predicate, which contains a date and time that must not be valid.
    pub not: Option<Not>,
    #[serde(rename = "abs_before")]
    /// A date and time that must be valid, .
    pub abs_before: Option<String>,
    /// A date and time that must be valid.
    #[serde(rename = "abs_before_epoch")]
    /// A date and time that must be valid.
    pub abs_before_epoch: Option<String>,
}

/// Represents a not predicate in the claimable balance record.
///
/// This struct defines the structure of a not predicate found in the claimable balance record
/// from the Horizon API. It contains the abs_before and abs_before_epoch fields of the not predicate.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Not {
    /// A date and time that must not be valid.
    #[serde(rename = "abs_before")]
    pub abs_before: String,
    /// A date and time that must not be valid.
    #[serde(rename = "abs_before_epoch")]
    pub abs_before_epoch: String,
}

/// Represents the Or predicate in the claimable balance record.
///
/// This struct defines the structure of an or predicate found in the claimable balance record
/// from the Horizon API. It contains the not, abs_before, and abs_before_epoch fields of the or predicate.
///
/// The Or predicate defines a set of date time conditions of which one has to be true in order for the given
/// date time to be valid
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Or {
    /// A not predicate. A Or predicate can have a Not predicate, which contains a date and time that must not be valid.
    #[serde(rename = "abs_before")]
    pub abs_before: Option<String>,
    #[serde(rename = "abs_before_epoch")]
    /// A date and time that must be valid.
    pub abs_before_epoch: Option<String>,
    /// A not predicate. A Or predicate can have a Not predicate, which contains a date and time that must not be valid.
    pub not: Option<Not>,
}

/// Represents the flags in the claimable balance record.
///
/// This struct defines the structure of the flags found in the claimable balance record
/// from the Horizon API. It contains the clawback_enabled field of the flags.
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
    /// Checks if the given date time is valid based on the predicate.
    pub fn is_valid(&self, date: DateTime<Utc>) -> bool {
        match self {
            // If the predicate is unconditional, return true
            Predicate {
                unconditional: Some(true),
                ..
            } => true,
            // If the predicate contains an and, check if all of the ands are valid
            Predicate {
                and: Some(ands), ..
            } => ands.iter().all(|cond| cond.is_valid(date)),
            // If the predicate contains an or, check if any of the ors are valid
            Predicate { or: Some(ors), .. } => ors.iter().any(|cond| cond.is_valid(date)),
            // If the predicate contains a not, check if the not is valid
            Predicate { not: Some(not), .. } => !not.is_valid(date),
            _ => false,
        }
    }
}

impl And {
    /// Checks if the given date time is valid based on the and predicate.
    fn is_valid(&self, date: DateTime<Utc>) -> bool {
        // If the and contains a not, check if the not is valid
        if let Some(not) = &self.not {
            if not.is_valid(date) {
                // If the not is valid, return false
                return false;
            }
        }
        // If the and contains an abs_before_epoch, check if the date is before the abs_before_epoch
        // If the date is before the abs_before_epoch, return true
        self.abs_before_epoch
            .as_ref()
            .map(|d| date < parse_epoch(d))
            .unwrap_or(true)
    }
}

impl Or {
    /// Checks if the given date time is valid based on the or predicate.
    fn is_valid(&self, date: DateTime<Utc>) -> bool {
        // If the or contains a not, check if the not is valid
        if let Some(not) = &self.not {
            if not.is_valid(date) {
                // If the not is valid,
                return true;
            }
        }
        // If the or contains an abs_before_epoch, check if the date is before the abs_before_epoch
        // If the date is before the abs_before_epoch, return true
        self.abs_before_epoch
            .as_ref()
            .map(|d| date < parse_epoch(d))
            .unwrap_or(false)
    }
}

impl Not {
    /// Checks if the given date time is valid based on the not predicate.
    fn is_valid(&self, date: DateTime<Utc>) -> bool {
        // If the not contains an abs_before_epoch, check if the date is before the abs_before_epoch
        // If the date is before the abs_before_epoch, return true (when this function returns true in the And and Or
        // is_valid functions, a false is returned.
        date <= parse_epoch(&self.abs_before_epoch)
    }
}

/// Parses the given epoch string into a DateTime<Utc>
fn parse_epoch(epoch_str: &str) -> DateTime<Utc> {
    // Convert the timestamp string into an i64
    let timestamp = epoch_str.parse::<i64>().unwrap();

    // Create a NaiveDateTime from the timestamp
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();

    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);

    return datetime;
}
