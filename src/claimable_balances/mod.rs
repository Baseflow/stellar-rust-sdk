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
/// # Usage
///
/// To use this module, create an instance of `AllClaimableBalancesRequest` and optionally set the desired
/// query parameters. Then, pass the request object to [`HorizonClient::get_all_claimable_balances`](crate::horizon_client::HorizonClient::get_all_claimable_balances)
/// method to execute the query. The method returns an `AllClaimableBalancesResponse` containing the information about
/// the requested assets.
///
/// # Example
/// ```
/// # use stellar_rust_sdk::claimable_balances::all_claimable_balances_request::AllClaimableBalancesRequest;
/// # use stellar_rust_sdk::horizon_client::HorizonClient;
/// # use stellar_rust_sdk::models::Request;
/// #
/// # async fn fetch_claimable_balances() -> Result<(), Box<dyn std::error::Error>> {
/// #     let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org".to_string())?;
/// let request = AllClaimableBalancesRequest::new()
///     .set_sponsor("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string())
///     .unwrap();
///
/// let response = horizon_client.get_all_claimable_balances(&request).await?;
/// // Process the response
/// #     Ok(())
/// # }
/// ```
///
pub mod all_claimable_balances_request;


/// Provides the `AllClaimableBalancesResponse` struct.
///
/// The `all_claimable_balances_response` module provides structures to parse and encapsulate 
/// the data returned by the Horizon server when a request for all claimable balances is made. 
/// Claimable balances are ledger entries that can be claimed by a designated account under 
/// certain conditions and are a unique feature of the Stellar network.
///
/// # Usage
///
/// This module is typically used by the `HorizonClient` to deserialize the JSON response from 
/// the Horizon server into a `AllClaimableBalancesResponse` object, which provides easy access 
/// to the list of claimable balances and related information.
///
/// # Example
/// ```
/// # use stellar_rust_sdk::claimable_balances::prelude::*;
/// # use stellar_rust_sdk::horizon_client::HorizonClient;
/// # use stellar_rust_sdk::models::Response;
/// #
/// # async fn get_all_claimable_balances() -> Result<(), Box<dyn std::error::Error>> {
/// #   let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org".to_string())?;
/// #   let request = AllClaimableBalancesRequest::new()
/// #       .set_sponsor("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string())
/// #       .unwrap();
/// #
/// let response: Result<AllClaimableBalancesResponse, String> = horizon_client
///     .get_all_claimable_balances(&request)
///     .await;
///
/// match response {
///     Ok(all_balances) => {
///         for balance in all_balances.embedded().records() {
///             println!("Balance ID: {}", balance.id());
///             // Further processing...
///         }
///     },
///     Err(e) => eprintln!("Error fetching claimable balances: {}", e),
/// }
/// #
/// #    Ok(())
/// # }
/// ```
///
/// The `all_claimable_balances_response` module is an essential part of the SDK that allows developers 
/// to work with claimable balances, supporting the building of applications that interact with 
/// this Stellar network feature.
pub mod all_claimable_balances_response;

pub mod single_claimable_balance_request;

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
/// use stellar_rust_sdk::claimable_balances::prelude::*;
///
/// // This enables direct use of AllClaimableBalancesRequest, AllClaimableBalancesResponse, etc.
/// let asset_request = AllClaimableBalancesRequest::new();
/// // Further usage...
/// ```
///
pub mod prelude {
    pub use super::all_claimable_balances_request::*;
    pub use super::all_claimable_balances_response::*;
    pub use super::single_claimable_balance_request::*;
    pub use super::single_claimable_balance_response::*;
}
