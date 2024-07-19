use chrono::DateTime;
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

/// Provides the claimable balance responses.
///
/// The `response` module provides structures to parse and encapsulate
/// the data returned by the Horizon server when a request for claimable balances is made.
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

    // Create a DateTime from the timestamp
    let datetime = DateTime::from_timestamp(timestamp, 0).unwrap();

    return datetime;
}

pub mod prelude {
    pub use super::{all_claimable_balances_request::*, response::*, single_claimable_balance_request::*};
}

#[cfg(test)]
mod tests {
    use super::parse_epoch;
    use super::prelude::*;
    use crate::{horizon_client::HorizonClient, Paginatable};
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

    #[tokio::test]
    async fn test_get_all_claimable_balances() {
        static ID: &str = "0000000010a8f6991f79df306f22a2032f6007ad594dd30f966b21556f7d75658ec1c4e9";
        static ASSET: &str = "native";
        static AMOUNT: &str = "3.0000000";
        static SPONSOR: &str = "GCRHSLTKEPLLRLC4MB5OJPO4DJYIMYHYBDHX4TET3XKUKFAYMWERHXVG";
        static LAST_MODIFIED_LEDGER: &i64 = &2170;
        static LAST_MODIFIED_TIME: &str = "2024-06-11T23:59:46Z";
        static CLAWBACK_ENABLED: &bool = &false;

        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let all_claimable_balances_request =
            AllClaimableBalancesRequest::new().set_limit(4).unwrap();

        let all_claimable_balances_response = horizon_client
            .get_all_claimable_balances(&all_claimable_balances_request)
            .await;

        assert!(all_claimable_balances_response.is_ok());

        let binding = all_claimable_balances_response.unwrap();
        let predicate = binding.embedded().records()[1].claimants()[0].predicate();

        let jan_first_2024 = Utc::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap();
        let valid_date = Utc::with_ymd_and_hms(&Utc, 2024, 2, 10, 0, 0, 0).unwrap();

        assert_eq!(predicate.is_valid(jan_first_2024), true);
        assert_eq!(predicate.is_valid(valid_date), true);
        let record = &binding.embedded().records()[0];

        assert_eq!(
            record.id(),
            ID
        );

        assert_eq!(
            record.asset(),
            ASSET
        );

        assert_eq!(record.amount(), AMOUNT);

        assert_eq!(
            record.sponsor(),
            SPONSOR
        );

        assert_eq!(record.last_modified_ledger(), LAST_MODIFIED_LEDGER);

        assert_eq!(
            record.last_modified_time().to_string(),
            LAST_MODIFIED_TIME
        );

        assert_eq!(record.flags().clawback_enabled(), CLAWBACK_ENABLED);
    }

    #[tokio::test]
    async fn test_get_single_claimable_balance() {
        static CLAIMABLE_BALANCE_ID: &str = "00000000fe3d8209ed9662e92f0d3a5c55068e18bd5e0697c3c6db6ac4c0870c6f3e0b38";
        static ID: &str = "00000000fe3d8209ed9662e92f0d3a5c55068e18bd5e0697c3c6db6ac4c0870c6f3e0b38";
        static ASSET: &str = "IOM:GBSUM7J4W2IH5LAMSQGI7Y2OZBV2BJB6EOK7TIK66DXNJUU4JAY36VR2";
        static AMOUNT: &str = "2.0000000";
        static SPONSOR: &str = "GA7UL5DDCP6WR7KV5GXKXSHBMP577U7TBDBTBY33J57RZE2A37KW67JB";
        static LAST_MODIFIED_LEDGER: &i64 = &9234;
        static LAST_MODIFIED_TIME: &str = "2024-06-12T10:19:12Z";
        static CLAWBACK_ENABLED: &bool = &false;
        static PAGING_TOKEN: &str = "9234-00000000fe3d8209ed9662e92f0d3a5c55068e18bd5e0697c3c6db6ac4c0870c6f3e0b38";

        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let single_claimable_balance_request = SingleClaimableBalanceRequest::new()
            .set_claimable_balance_id(CLAIMABLE_BALANCE_ID.to_string());

        let single_claimable_balance_response = horizon_client
            .get_single_claimable_balance(&single_claimable_balance_request)
            .await;

        assert!(single_claimable_balance_response.is_ok());

        let binding = single_claimable_balance_response.clone().unwrap();
        let predicate = binding.claimants()[0].predicate();

        let jan_first_2024 = Utc::with_ymd_and_hms(&Utc, 2021, 1, 1, 0, 0, 0).unwrap();
        let valid_date = Utc::with_ymd_and_hms(&Utc, 2021, 1, 1, 0, 0, 0).unwrap();

        assert_eq!(predicate.is_valid(jan_first_2024), true);
        assert_eq!(predicate.is_valid(valid_date), true);

        let single_claimable_balance_response = single_claimable_balance_response.unwrap();
        assert_eq!(
            single_claimable_balance_response.id().to_string(),
            ID
        );

        assert_eq!(
            single_claimable_balance_response.asset().to_string(),
            ASSET);

        assert_eq!(
            single_claimable_balance_response.amount().to_string(),
            AMOUNT);

        assert_eq!(
            single_claimable_balance_response.sponsor().to_string(),
            SPONSOR);

        assert_eq!(
            single_claimable_balance_response.last_modified_ledger(),
            LAST_MODIFIED_LEDGER);

        assert_eq!(
            single_claimable_balance_response
                .last_modified_time()
                .to_string(),
            LAST_MODIFIED_TIME);

        assert_eq!(
            single_claimable_balance_response.flags().clawback_enabled(),
            CLAWBACK_ENABLED);

        assert_eq!(
            single_claimable_balance_response.paging_token().to_string(),
            PAGING_TOKEN);
    }
}
