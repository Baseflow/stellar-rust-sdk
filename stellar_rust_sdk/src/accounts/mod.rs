/// Provides the `AccountsRequest`.
///
/// This module provides the `AccountsRequest` struct, which is specifically designed
/// for constructing requests to query for multiple accounts from the Horizon server. It is
/// is meant to be used with the [`HorizonClient::get_accounts_list`](crate::horizon_client::HorizonClient::get_account_list)
/// method.
///
pub mod accounts_request;

/// Provides the `SingleAccountRequest`.
///
/// This module provides the `SingleAccountRequest` struct, specifically designed for
/// constructing requests to query information about a single account from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_single_account`](crate::horizon_client::HorizonClient::get_single_account)
/// method.
///
pub mod single_account_request;

/// Provides the `Responses`.
///
/// This module defines structures representing the response from the Horizon API when querying
/// for accounts. The structures are designed to deserialize the JSON response into Rust
/// objects, enabling straightforward access to various details of Stellar accounts.
///
/// These structures are equipped with serialization capabilities to handle the JSON data from the
/// Horizon server and with getter methods for easy field access.
///
pub mod response;

/// The base path for account-related endpoints in the Horizon API.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for account-related API calls.
/// This specific variable is made public within the crate, since the `offers` endpoint needs this
/// to construct an url.
pub(crate) static ACCOUNTS_PATH: &str = "accounts";

/// The `prelude` module of the `accounts` module.
///
/// This module serves as a convenience for users of the Horizon Rust SDK, allowing for easy and
/// ergonomic import of the most commonly used items across various modules. It re-exports
/// key structs and traits from the sibling modules, simplifying access to these components
/// when using the library.
///
/// By importing the contents of `prelude`, users can conveniently access the primary
/// functionalities of the accounts-related modules without needing to import each item
/// individually.
///
/// # Contents
///
/// The `prelude` includes the following re-exports:
///
/// * From `accounts_request`: All items (e.g., `AccountsRequest`).
/// * From `accounts_response`: All items (e.g., `AccountsResponse`, `Record`, etc.).
/// * From `single_account_request`: All items (e.g., `SingleAccountRequest`).
/// * From `single_account_response`: All items (e.g., `SingleAccountResponse`, `Balance`, etc.).
///
/// # Example
/// ```
/// # use crate::stellar_rs::models::*;
/// // Import the contents of the account prelude
/// use stellar_rs::accounts::prelude::*;
///
/// // Now you can directly use AccountsRequest, SingleAccountResponse, etc.
/// let account_request = AccountsRequest::new();
/// ```
///
pub mod prelude {
    pub use super::accounts_request::*;
    pub use super::response::*;
    pub use super::single_account_request::*;
}

#[cfg(test)]
pub mod test {

    use super::prelude::*;
    use crate::horizon_client::HorizonClient;

    static SEQUENCE: &str = "131988639973376";
    static ACCOUNT_ID: &str = "GCAHCEGRUI7FFAQE3DBQWV7ULMQHFBUIVRZC4R2VISREAY6D52Z2NODN";
    static LAST_MODIFIED_TIME: &str = "2024-02-08T14:25:14Z";
    static LAST_MODIFIED_LEDGER: u64 = 30731;

    #[tokio::test]
    async fn test_get_account_list() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let accounts_request = AccountsRequest::new()
            .set_signer_filter(ACCOUNT_ID)
            .unwrap()
            .set_limit(10)
            .unwrap();

        // call the get_account_list method to retrieve the account list response
        let accounts_response: Result<AccountsResponse, String> =
            horizon_client.get_account_list(&accounts_request).await;

        assert!(accounts_response.is_ok());
        let binding = accounts_response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(response.account_id(), ACCOUNT_ID);
        assert_eq!(response.id(), ACCOUNT_ID);
        assert_eq!(response.sequence(), SEQUENCE);
        assert_eq!(response.subentry_count(), &0);
        assert_eq!(response.last_modified_ledger(), &LAST_MODIFIED_LEDGER);
        assert_eq!(response.last_modified_time(), LAST_MODIFIED_TIME);
        assert_eq!(response.thresholds().low_threshold(), &0);
        assert_eq!(response.thresholds().med_threshold(), &0);
        assert_eq!(response.thresholds().high_threshold(), &0);
        assert_eq!(response.flags().auth_required(), &false);
        assert_eq!(response.flags().auth_revocable(), &false);
        assert_eq!(response.flags().auth_immutable(), &false);
        assert_eq!(response.flags().auth_clawback_enabled(), &false);
        assert_eq!(response.balances()[0].balance(), "10000.0000000");
        assert_eq!(response.balances()[0].asset_type(), "native");
        assert_eq!(response.balances()[0].buying_liabilities(), "0.0000000");
        assert_eq!(response.balances()[0].selling_liabilities(), "0.0000000");
        assert_eq!(response.signers()[0].key(), ACCOUNT_ID);
        assert_eq!(response.signers()[0].weight(), &1);
        assert_eq!(response.signers()[0].singer_type(), "ed25519_public_key");
        assert_eq!(response.num_sponsoring(), &0);
        assert_eq!(response.num_sponsored(), &0);
        assert_eq!(response.paging_token(), ACCOUNT_ID);
    }

    #[tokio::test]
    async fn test_get_single_account() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let single_account_request = SingleAccountRequest::new()
            .set_account_id(ACCOUNT_ID.to_string())
            .unwrap();

        let single_account_response = horizon_client
            .get_single_account(&single_account_request)
            .await;

        assert!(single_account_response.is_ok());
        let response = single_account_response.unwrap();
        assert_eq!(response.account_id().to_string(), ACCOUNT_ID);
        assert_eq!(response.sequence().to_string(), SEQUENCE);
        assert_eq!(response.subentry_count(), &0);
        assert_eq!(response.last_modified_ledger(), &LAST_MODIFIED_LEDGER);
        assert_eq!(response.last_modified_time(), LAST_MODIFIED_TIME);
        assert_eq!(response.thresholds().low_threshold(), &0);
        assert_eq!(response.thresholds().med_threshold(), &0);
        assert_eq!(response.thresholds().high_threshold(), &0);
        assert_eq!(response.flags().auth_required(), &false);
        assert_eq!(response.flags().auth_revocable(), &false);
        assert_eq!(response.flags().auth_immutable(), &false);
        assert_eq!(response.flags().auth_clawback_enabled(), &false);
        assert_eq!(response.balances()[0].balance(), "10000.0000000");
        assert_eq!(response.balances()[0].asset_type(), "native");
        assert_eq!(response.balances()[0].buying_liabilities(), "0.0000000");
        assert_eq!(response.balances()[0].selling_liabilities(), "0.0000000");
        assert_eq!(response.signers()[0].key(), ACCOUNT_ID);
        assert_eq!(response.signers()[0].weight(), &1);
        assert_eq!(response.signers()[0].singer_type(), "ed25519_public_key");
        assert_eq!(response.num_sponsoring(), &0);
        assert_eq!(response.num_sponsored(), &0);
        assert_eq!(response.paging_token(), ACCOUNT_ID);
    }
}
