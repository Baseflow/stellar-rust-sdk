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

/// Provides the Asset responses.
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
    use crate::{horizon_client::HorizonClient, Paginatable};

    #[tokio::test]
    async fn test_get_all_assets() {
        static ASSET_TYPE: &str = "credit_alphanum4";
        static ASSET_CODE: &str = "006";
        static ASSET_ISSUER: &str = "GCN4ALWVHURX3D64AQ3PD7VFOLLKHMEFOG3Y4A4DKOTV256IZNJG3IKB";
        static PAGING_TOKEN: &str =
            "006_GCN4ALWVHURX3D64AQ3PD7VFOLLKHMEFOG3Y4A4DKOTV256IZNJG3IKB_credit_alphanum4";
        static NUM_ACCOUNTS: &u32 = &1;
        static NUM_CLAIMABLE_BALANCES: &u32 = &0;
        static NUM_LIQUIDITY_POOLS: &u32 = &0;

        static AMOUNT: &str = "999.0000000";
        static AUTHORIZED: &u32 = &1;
        static AUTHORIZED_TO_MAINTAIN_LIABILITIES: &u32 = &0;
        static UNAUTHORIZED: &u32 = &0;
        static CLAIMABLE_BALANCES_AMOUNT: &str = "0.0000000";
        static LIQUIDITY_POOLS_AMOUNT: &str = "0.0000000";
        static CONTRACTS_AMOUNT: &str = "0.0000000";
        static BALANCES_AUTHORIZED: &str = "999.0000000";
        static BALANCES_UNAUTHORIZED: &str = "0.0000000";
        static AUTH_REQUIRED: &bool = &false;
        static AUTH_REVOCABLE: &bool = &false;
        static AUTH_IMMUTABLE: &bool = &false;
        static AUTH_CLAWBACK_ENABLED: &bool = &false;


        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let all_assets_request: AllAssetsRequest = AllAssetsRequest::new().set_limit(1).unwrap();

        let response = horizon_client.get_all_assets(&all_assets_request).await;

        assert!(response.is_ok());
        let binding = response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(response.asset_type(), ASSET_TYPE);
        assert_eq!(response.asset_code(), ASSET_CODE);
        assert_eq!(response.asset_issuer(), ASSET_ISSUER);
        assert_eq!(response.paging_token(), PAGING_TOKEN);
        assert_eq!(
            response.paging_token(),
            &format!("{}_{}_{}", ASSET_CODE, ASSET_ISSUER, ASSET_TYPE)
        );
        assert_eq!(response.num_accounts(), NUM_ACCOUNTS);
        assert_eq!(response.num_claimable_balances(), NUM_CLAIMABLE_BALANCES);
        assert_eq!(response.num_liquidity_pools(), NUM_LIQUIDITY_POOLS);
        assert_eq!(response.amount(), AMOUNT);
        assert_eq!(response.accounts().authorized(), AUTHORIZED);
        assert_eq!(response.accounts().authorized_to_maintain_liabilities(), AUTHORIZED_TO_MAINTAIN_LIABILITIES);
        assert_eq!(response.accounts().unauthorized(), UNAUTHORIZED);
        assert_eq!(response.claimable_balances_amount(), CLAIMABLE_BALANCES_AMOUNT);
        assert_eq!(response.liquidity_pools_amount(), LIQUIDITY_POOLS_AMOUNT);
        assert_eq!(response.contracts_amount(), CONTRACTS_AMOUNT);
        assert_eq!(response.balances().authorized(), BALANCES_AUTHORIZED);
        assert_eq!(response.balances().unauthorized(), BALANCES_UNAUTHORIZED);
        assert_eq!(response.flags().auth_required(), AUTH_REQUIRED);
        assert_eq!(response.flags().auth_revocable(), AUTH_REVOCABLE);
        assert_eq!(response.flags().auth_immutable(), AUTH_IMMUTABLE);
        assert_eq!(response.flags().auth_clawback_enabled(), AUTH_CLAWBACK_ENABLED);
    }
}
