use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;

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
pub mod response;

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
    pub use super::response::*;
    pub use super::single_claimable_balance_request::*;
}

#[cfg(test)]
mod tests {
    use super::parse_epoch;
    use super::prelude::*;
    use chrono::DateTime;
    use chrono::{TimeZone, Utc};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref DATE: DateTime<Utc> =
            Utc::with_ymd_and_hms(&Utc, 2021, 9, 30, 18, 40, 0).unwrap();
        static ref DATE_AND_ONE_SECOND: chrono::DateTime<Utc> =
            Utc::with_ymd_and_hms(&Utc, 2021, 9, 30, 18, 40, 1).unwrap();
        static ref EPOCH_STR: String = "1633027200".to_string();
    }

    #[test]
    fn test_and_is_valid() {
        let and = And {
            not: Some(Not {
                abs_before: EPOCH_STR.to_string(),
                abs_before_epoch: EPOCH_STR.to_string(),
            }),
            abs_before: None,
            abs_before_epoch: None,
        };
        assert_eq!(and.is_valid(*DATE), false);
    }

    #[test]
    fn test_or_is_valid() {
        let or = Or {
            not: Some(Not {
                abs_before: EPOCH_STR.to_string(),
                abs_before_epoch: EPOCH_STR.to_string(),
            }),
            abs_before: None,
            abs_before_epoch: None,
        };
        assert_eq!(or.is_valid(*DATE), true);
    }

    #[test]
    fn test_not_is_valid() {
        let not = Not {
            abs_before: EPOCH_STR.to_string(),
            abs_before_epoch: EPOCH_STR.to_string(),
        };
        assert_eq!(not.is_valid(*DATE_AND_ONE_SECOND), false);
    }

    #[test]
    fn test_parse_epoch() {
        assert_eq!(parse_epoch(&EPOCH_STR.to_string()), *DATE);
    }
}
