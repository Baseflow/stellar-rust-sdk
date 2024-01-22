use chrono::{DateTime, NaiveDateTime, Utc};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::models::Response;


/// Represents the reponse toa request for listing all claimable balances from the Stellar Horizon API.
///
/// This struct contains the overall structure of the response for querying all claimable balances. It includes
/// navigational links and a collection of claimable balance records, each with comprehensive details about the claimable balance.
///
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableBalancesResponse {
    #[serde(rename = "_links")]
    pub links: AllClaimableBalancesResponseLink,
    #[serde(rename = "_embedded")]
    pub embedded: Embedded,
}

/// Represents the navigational links in a all claimable balances response from the Stellar Horizon API.
///
/// This struct includes links such as the self-link (current page), next, and previous,
/// providing quick navigation across different pages of the all claimable balances response.
///
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllClaimableBalancesResponseLink {
    /// A self-link to the current page of the all claimable balances response.
    #[serde(rename = "self")]
    pub self_field: Option<SelfLink>,
    /// A link to the next page of the all claimable balances response.
    pub next: Option<SelfLink>,
    /// A link to the previous page of the all claimable balances response.
    pub prev: Option<SelfLink>,
}

/// Represents a self-link in the all claimable balances response
///
/// This struct defines the structure of a self-link (`href`) found in the all claimabale balances response
/// from the Horizon API. It contains the URL tot eh current resource or query.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct SelfLink {
    /// A `String` representing the hyperlink reference to the current resource or query.
    pub href: String,
}

/// Contains the embedded claimbable balance records in the all claimable balances response.
///
/// This struct encapsulates a collection of all clamaimable balance records (`Records`) returned in the response.
/// It provides a way to access each individual claimable balance record and its detailed information.
///
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    /// A vector of individual claimable balance records.
    pub records: Vec<Record>,
}

/// Represents a single claimable balance record in the all claimable balances response.
///
/// This struct encapsulates detailed information about a claimable balance, including its ID, asset, amount, sponsor,
/// last modified claimable balance, last modified time, claimants, flags, and paging token.
///
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(rename = "_links")]
    pub links: RecordLinks,
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

/// Represents the navigational links in a claimable balance record from the Stellar Horizon API.
///
/// This struct includes links such as the self-link (current page), transactions, and operations,
/// providing quick navigation across different pages of the claimable balance record.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct RecordLinks {
    /// A self-link to the current page of the claimable balance record.
    #[serde(rename = "self")]
    pub self_field: SelfLink,
    /// A link to the transactions related to the claimable balance record.
    pub transactions: Transactions,
    /// A link to the operations related to the claimable balance record.
    pub operations: Operations,
}

/// Represents a link to the transactions related to the claimable balance record.
///
/// This struct defines the structure of a transactions link (`href`) found in the claimable balance record
/// from the Horizon API. It contains the URL to the transactions related to the claimable balance record.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    /// A `String` representing the hyperlink reference to the transactions related to the claimable balance record.
    pub href: String,
    /// A `bool` indicating whether the transactions link is templated.
    pub templated: bool,
}

/// Represents a link to the operations related to the claimable balance record.
///
/// This struct defines the structure of an operations link (`href`) found in the claimable balance record
/// from the Horizon API. It contains the URL to the operations related to the claimable balance record.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Operations {
    /// A `String` representing the hyperlink reference to the operations related to the claimable balance record.
    pub href: String,
    /// A `bool` indicating whether the operations link is templated.
    pub templated: bool,
}

/// Represents a claimant in the claimable balance record.
///
/// This struct defines the structure of a claimant found in the claimable balance record
/// from the Horizon API. It contains the destination and predicate of the claimant.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Claimant {
    /// The destination of the claimant.
    pub destination: String,
    /// The predicate of the claimant.
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
    /// A date and time that must be valid.
    pub abs_before: Option<String>,
    /// A date and time that must be valid.
    #[serde(rename = "abs_before_epoch")]
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
    /// A date and time that must be valid.
    #[serde(rename = "abs_before_epoch")]
    pub abs_before_epoch: Option<String>,
    /// A not predicate. A Or predicate can have a Not predicate, which contains a date and time that must not be valid.
    pub not: Option<Not>,
}

/// Represents the flags in the claimable balance record.
///
/// This struct defines the structure of the flags found in the claimable balance record
/// from the Horizon API. It contains the clawback_enabled field of the flags.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
    /// A `bool` indicating whether the clawback is enabled.
    #[serde(rename = "clawback_enabled")]
    pub clawback_enabled: bool,
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

impl Response for AllClaimableBalancesResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}