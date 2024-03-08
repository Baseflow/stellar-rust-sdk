/// Provides the `AllAssetsRequest` struct.
///
/// This module contains the `AllAssetsRequest` struct, which is designed to create requests
/// for querying comprehensive lists of assets from the Horizon server. It facilitates specifying
/// various parameters to tailor the asset query, such as asset issuer, asset code, and pagination options.
///
/// The `AllAssetsRequest` struct is meant to be used in conjunction with the [`HorizonClient`](crate::horizon_client::HorizonClient)
/// to perform the actual API calls and fetch asset data. It adheres to the structure
/// and requirements of the Horizon API for asset queries.
///
pub mod all_assets_request;

/// Provides the `AllAssetsResponse`.
///
/// This module offers a set of structures representing the response received from the Horizon
/// API when querying for all assets. These structures are tailored to convert the JSON response
/// from the Horizon server into structured Rust objects, making it easier to handle and utilize
/// asset data within client applications.
/// For a detailed description of the response structure, refer to the
/// <a href="https://developers.stellar.org/api/horizon/resources/list-all-assets">List All Assets endpoint</a> documentation.
///
/// The provided structures include serialization and deserialization capabilities for dealing
/// with the JSON format of the Horizon server's response. They also leverage the `Getters` derive macro
/// to furnish convenient getter methods for accessing their fields.
///
pub mod response;

/// The base path for all assets endpoints in the Stellar Horizon API.
///
/// This static variable holds the string slice that represents the common base path used in constructing
/// URLs for asset-related queries to the Horizon server. It forms a constant part of the route for all
/// asset-related API endpoints, ensuring uniformity in URL construction across different parts of the SDK.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for asset-related API calls.
///
static ASSET_PATH: &str = "assets";

/// The `prelude` module of the `assets` module.
///
/// This module is designed as a convenience for users of the Stellar Horizon Rust SDK, facilitating
/// easy and ergonomic imports of commonly used items related to asset data. It re-exports essential
/// structs and traits from the sibling modules in `assets`, streamlining access to these components
/// when utilizing the SDK in client applications.
///
/// By importing from `prelude`, users gain immediate access to the primary functionalities of the
/// asset-related modules without the need for importing each item individually, simplifying code
/// and improving readability.
///
/// # Contents
///
/// The `prelude` includes the following re-exports:
///
/// * From `all_assets_request`: All items (e.g., `AllAssetsRequest`).
/// * From `all_assets_response`: All items (e.g., `AllAssetsResponse`, `Records`, etc.).
///
/// This approach allows for a more concise and focused usage pattern, especially beneficial
/// when dealing with multiple components related to asset data in the Horizon API.
///
/// # Example
/// ```
/// // Import the contents of the assets prelude
/// use stellar_rs::assets::prelude::*;
///
/// // This enables direct use of AllAssetsRequest, AllAssetsResponse, etc.
/// let asset_request = AllAssetsRequest::new();
/// // Further usage...
/// ```
///
pub mod prelude {
    pub use super::all_assets_request::*;
    pub use super::response::*;
}

#[cfg(test)]
pub mod test {

    use super::prelude::*;
    use crate::horizon_client::HorizonClient;

    #[tokio::test]
    async fn test_get_all_assets() {
        let asset_type = "credit_alphanum4";
        let asset_code = "0";
        let asset_issuer = "GAGNEED7RUE6PNAB3AKXFU6QZF4EUSVTICHE7YRHB53KDOEHGKWBL6BE";
        let paging_token =
            "0_GAGNEED7RUE6PNAB3AKXFU6QZF4EUSVTICHE7YRHB53KDOEHGKWBL6BE_credit_alphanum4";
        let num_accounts = 0;
        let amount = "0.0000000";
        let num_authorized = 0;
        let num_unauthorized = 0;
        let balances_authorized = "0.0000000";
        let balances_unauthorized = "0.0000000";

        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let all_assets_request: AllAssetsRequest = AllAssetsRequest::new().set_limit(1).unwrap();

        let response = horizon_client.get_all_assets(&all_assets_request).await;

        assert!(response.is_ok());
        let binding = response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(response.asset_type(), asset_type);
        assert_eq!(response.asset_code(), asset_code);
        assert_eq!(response.asset_issuer(), asset_issuer);
        assert_eq!(response.paging_token(), paging_token);
        assert_eq!(
            response.paging_token(),
            &format!("{}_{}_{}", asset_code, asset_issuer, asset_type)
        );
        assert_eq!(response.num_accounts(), &num_accounts);
        assert_eq!(response.num_claimable_balances(), &0);
        assert_eq!(response.num_liquidity_pools(), &0);
        assert_eq!(response.amount(), amount);
        assert_eq!(response.accounts().authorized(), &num_authorized);
        assert_eq!(response.accounts().authorized_to_maintain_liabilities(), &2);
        assert_eq!(response.accounts().unauthorized(), &num_unauthorized);
        assert_eq!(response.claimable_balances_amount(), "0.0000000");
        assert_eq!(response.liquidity_pools_amount(), "0.0000000");
        assert_eq!(response.contracts_amount(), "0.0000000");
        assert_eq!(response.balances().authorized(), balances_authorized);
        assert_eq!(
            response.balances().authorized_to_maintain_liabilities(),
            "1.0000000"
        );
        assert_eq!(response.balances().unauthorized(), balances_unauthorized);

        let auth_required = true;
        assert_eq!(response.flags().auth_required(), &auth_required);
        assert_eq!(response.flags().auth_revocable(), &true);
        assert_eq!(response.flags().auth_immutable(), &false);
        assert_eq!(response.flags().auth_clawback_enabled(), &true);
    }
}
