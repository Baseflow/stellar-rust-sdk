use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use derive_getters::Getters;

use serde::{Deserialize, Serialize};
/// Provides the `AllClaimableBalancesRequest` struct.
///
/// This module contains the `AllClaimableBalancesRequest` struct, which is designed to create requests
/// for querying comprehensive lists of claimable balances from the Horizon server. It facilitates specifying
/// various parameters to tailor the query, such as sponsor, asset, claimant, and pagination options.
///
/// The `AllClaimableBalancesRequest` struct is meant to be used in conjunction with the
/// [`HorizonClient`](crate::horizon_client::HorizonClient)
/// to perform the actual API calls and fetch claimable balance data. It adheres to the structure
/// and requirements of the Horizon API for claimable balance queries.
///
pub mod all_claimable_balances_request;

/// Provides the `AllClaimableBalancesResponse` struct.
///
/// The `all_claimable_balances_response` module provides structures to parse and encapsulate
/// the data returned by the Horizon server when a request for all claimable balances is made.
/// Claimable balances are ledger entries that can be claimed by a designated account under
/// certain conditions and are a unique feature of the Stellar network.
///
pub mod all_claimable_balances_response;

/// Provides the `SingleClaimableBalanceRequest` struct.
///
/// This module contains the `SingleClaimableBalanceRequest` struct, which is utilized to create
/// requests for retrieving information about a single claimable balance from the Stellar Horizon API.
/// It is specifically designed to query detailed data for a particular claimable balance identified by its ID.
///
/// The struct is intended to be used with the [`HorizonClient`](crate::horizon_client::HorizonClient)
/// to perform API calls and fetch detailed information about a specific claimable balance.

///
pub mod single_claimable_balance_request;

/// Provides the `SingleClaimableBalanceResponse`.
///
/// This module contains structures representing the response received from the Horizon API
/// when querying a single claimable balance. The main structure, `SingleClaimableBalanceResponse`,
/// is designed to convert the JSON response from the Horizon server into structured Rust objects.
/// This allows for easier handling and utilization of claimable balance data within client applications.
///
/// For a detailed description of the response structure, refer to the
/// [Retrieve a Single Claimable Balance](https://developers.stellar.org/api/horizon/resources/retrieve-a-claimable-balance)
/// endpoint documentation on the Stellar Developer's site.
///
/// The structures in this module include serialization and deserialization capabilities to handle
/// JSON data returned by the Horizon server. The `Getters` derive macro is used to provide
/// convenient getter methods for accessing fields of these structures.
///
pub mod single_claimable_balance_response;

/// The base path for all claimable balance related endpoints in the Stellar Horizon API.
///
/// This static variable holds the string slice that represents the common base path used in constructing
/// URLs for claimable-balance-related queries to the Horizon server. It forms a constant part of the route for all
/// claimable-balance-related API endpoints, ensuring uniformity in URL construction across different parts of the SDK.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for claimable-balance-related API calls.
///
static CLAIMABLE_BALANCES_PATH: &str = "claimable_balances";

/// The `prelude` module of the `claimable_balance` module.
///
/// This module is designed as a convenience for users of the Stellar Horizon Rust SDK, facilitating
/// easy and ergonomic imports of commonly used items related to claimable balance data. It re-exports essential
/// structs and traits from the sibling modules in `claimable_balances`, streamlining access to these components
/// when utilizing the SDK in client applications.
///
/// By importing from `prelude`, users gain immediate access to the primary functionalities of the
/// claimable-balance-related modules without the need for importing each item individually, simplifying code
/// and improving readability.
///
/// # Contents
///
/// The `prelude` includes the following re-exports:
///
/// * From `all_claimable_balances_request`: All items (e.g., `AllClaimableBalancesRequest`).
/// * From `all_claimable_balances_response`: All items (e.g., `AllClaimableBalancesResponse`, `Record`, etc.).
///
/// This approach allows for a more concise and focused usage pattern, especially beneficial
/// when dealing with multiple components related to asset data in the Horizon API.
///
/// # Example
/// ```
/// // Import the contents of the claimable_balances prelude
/// use stellar_rs::claimable_balances::prelude::*;
///
/// // This enables direct use of AllClaimableBalancesRequest, AllClaimableBalancesResponse, etc.
/// let asset_request = AllClaimableBalancesRequest::new();
/// // Further usage...
/// ```
///
///

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
    #[serde(rename = "clawback_enabled")]
    pub clawback_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase", rename(serialize = "self_field"))]
pub struct SelfField {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Next {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Prev {
    pub href: String,
}

/// Represents a link to the transactions of a claimable balance.
#[derive(Default, Debug, Clone, Serialize, PartialEq, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    /// The URL of the transactions link.
    pub href: String,

    /// Indicates if the link is templated.
    pub templated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct NavigationLinks {
    #[serde(rename = "self")]
    pub self_field: SelfField,
    pub next: Next,
    pub prev: Prev,
}

/// Contains navigational links related to the single claimable balance response.
#[derive(Default, Debug, Clone, Serialize, PartialEq, Deserialize, Getters)]
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

/// Represents a link to the operations of a claimable balance.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Operations {
    /// The URL of the operations link.
    pub href: String,

    /// Indicates if the link is templated.
    pub templated: bool,
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

pub mod prelude {
    pub use super::all_claimable_balances_request::*;
    pub use super::all_claimable_balances_response::*;
    pub use super::single_claimable_balance_request::*;
    pub use super::single_claimable_balance_response::*;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_and_is_valid() {
        let and = And {
            not: Some(Not {
                abs_before: "1633027200".to_string(),
                abs_before_epoch: "1633027200".to_string(),
            }),
            abs_before: None,
            abs_before_epoch: None,
        };
        let date = Utc::with_ymd_and_hms(&Utc, 2021, 9, 30, 18, 40, 0).unwrap();
        assert_eq!(and.is_valid(date), false);
    }

    #[test]
    fn test_or_is_valid() {
        let or = Or {
            not: Some(Not {
                abs_before: "1633027200".to_string(),
                abs_before_epoch: "1633027200".to_string(),
            }),
            abs_before: None,
            abs_before_epoch: None,
        };
        let date = Utc::with_ymd_and_hms(&Utc, 2021, 9, 30, 18, 40, 0).unwrap();

        assert_eq!(or.is_valid(date), true);
    }

    #[test]
    fn test_not_is_valid() {
        let not = Not {
            abs_before: "1633027200".to_string(),
            abs_before_epoch: "1633027200".to_string(),
        };
        let date = Utc::with_ymd_and_hms(&Utc, 2021, 9, 30, 18, 40, 1).unwrap();

        assert_eq!(not.is_valid(date), false);
    }

    #[test]
    fn test_parse_epoch() {
        let epoch_str = "1633027200";
        let expected_date = Utc::with_ymd_and_hms(&Utc, 2021, 9, 30, 18, 40, 0).unwrap();

        assert_eq!(parse_epoch(epoch_str), expected_date);
    }
}
