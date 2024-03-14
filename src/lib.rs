//! Stellar Horizon SDK for Rust
//!
//! This Rust library provides a user-friendly interface to the Stellar Horizon API,
//! allowing developers to easily query and transact on the Stellar network. Centered
//! around the `HorizonClient`, the SDK abstracts the underlying HTTP request and response
//! mechanisms into a set of simple, high-level methods.
//!
//! The SDK is designed with a focus on developer experience, providing clear abstractions,
//! sensible defaults, and streamlined error handling.
//!
//! ## Status
//!
//! The SDK is under active development. It is functional but should be considered a
//! work-in-progress. Features may be added or changed, and the SDK may evolve before
//! stabilization.
//!
//! #### Supported endpoints:
//! ![33%](https://progress-bar.dev/33/?width=200)
//! * Accounts
//! * Assets
//! * Effects
//! * Claimable balance
//! * Ledgers
//!
//! #### Endpoints on the roadmap:
//! * Fee stats
//! * Liquidity pools
//! * Offers
//! * Operations
//! * Orderbook
//! * Paths
//! * Payments
//! * Trade aggregations
//! * Trades
//! * Transactions

//!
//! ## Example Usage
//!
//! The following example demonstrates how to use the `HorizonClient` to retrieve a list
//! of accounts with a specific signer:
//!
//! ```rust
//! use stellar_rs::horizon_client::HorizonClient;
//! use stellar_rs::accounts::prelude::{AccountsRequest, AccountsResponse};
//! use stellar_rs::models::{Request, Response};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize the Horizon client with the testnet server
//!     let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org".to_string())?;
//!
//!     // Create a request to fetch accounts with a specific signer
//!     let accounts_request = AccountsRequest::new()
//!         .set_signer_filter("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7")?
//!         .set_limit(10)?;
//!
//!     // Perform the request using the Horizon client
//!     let accounts_response =
//!         horizon_client.get_account_list(&accounts_request)
//!         .await;
//!
//!     // Check for success and handle the response or error
//!     match accounts_response {
//!         Ok(response) => {
//!             // Process the response
//!         },
//!         Err(e) => {
//!             // Handle the error
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! This example initializes a `HorizonClient`, constructs an `AccountsRequest` to filter
//! accounts by signer, and calls `get_account_list` to retrieve the relevant data.
//! The result is then handled in a match expression, demonstrating the SDK's straightforward
//! error handling.
//!
//! Visit the documentation for `HorizonClient` and endpoint-specific request and response
//! types for more examples and detailed usage instructions.

use derive_getters::Getters;
/// Provides `Request` and `Response` structs for retrieving accounts.
///
/// This module provides a set of specialized request and response structures designed for
/// interacting with the accounts-related endpoints of the Horizon server. These structures
/// facilitate the construction of requests to query account data and the interpretation of
/// the corresponding responses.
///
/// # Usage
///
/// This module is intended to be used in conjunction with the [`HorizonClient`](crate::horizon_client::HorizonClient)
/// for making specific account-related API calls to the Horizon server. The request
/// structures are designed to be passed to the client's methods, which handle the
/// communication with the server and return the corresponding response structures.
///
/// # Example
/// An example of retrieving a list of accounts, filtering by signer:
/// ```rust
/// # use stellar_rs::accounts::prelude::{AccountsRequest, AccountsResponse};
/// # use stellar_rs::models::Request;
/// # use stellar_rs::horizon_client::HorizonClient;
/// #
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let base_url = "https://horizon-testnet.stellar.org".to_string();
/// # let horizon_client = HorizonClient::new(base_url)
/// #    .expect("Failed to create Horizon Client");
/// let request = AccountsRequest::new()
///     .set_signer_filter("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7").unwrap()
///     .set_limit(10).unwrap();
///
/// let response = horizon_client
///     .get_account_list(&request)
///     .await?;
/// # Ok({})
/// # }
///
pub mod accounts;

/// Provides `Request` and `Response` structs for retrieving assets.
///
/// This module provides the structures and functionalities necessary to interact with asset-related
/// endpoints of the Stellar Horizon API. It defines the request and response handlers for querying
/// information about assets on the Stellar network as described in the Stellar Horizon API documentation
/// on [Assets](https://developers.stellar.org/api/horizon/resources/assets). It is intended to be used in
/// conjunction with the is intended to be used in conjunction with the [`HorizonClient`](crate::horizon_client::HorizonClient)
/// struct.
///
/// # Example
///
/// The `assets` module simplifies the process of constructing queries about assets and interpreting the results. For example:
///
/// ```rust
/// # use stellar_rs::assets::prelude::*;
/// # use stellar_rs::models::Request;
/// # use stellar_rs::horizon_client::HorizonClient;
/// #
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let base_url = "https://horizon-testnet.stellar.org".to_string();
/// # let horizon_client = HorizonClient::new(base_url)
/// #    .expect("Failed to create Horizon Client");
/// let request = AllAssetsRequest::new()
///     .set_asset_code("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7").unwrap();
///
/// let response = horizon_client
///     .get_all_assets(&request)
///     .await?;
/// # Ok({})
/// # }
/// ```
///
pub mod assets;

/// Provides `Request` and `Response` structs for retrieving claimable balances.
///
/// This module provides structures and functionalities related to claimable balances within the Stellar network.
/// Claimable balances are a feature of the Stellar network that allows for the creation of balances that are
/// claimable by a designated party. They are used to facilitate payments to accounts that may not yet exist
/// or to provide an assurance that funds can be claimed by the rightful recipient.
///
/// The module comprises request and response structs for both single and batched operations involving
/// claimable balances. These are designed to interface with the Horizon API's endpoints for creating,
/// retrieving, and claiming such balances.
///
/// # Usage
///
/// To utilize the functionalities for claimable balances, import the necessary structs from this module
/// and use them to interact with the Horizon API. The `HorizonClient` methods, such as `get_claimable_balances`
/// and `get_claimable_balance`, will typically return the response structs provided here.
///
/// # Example
/// ```rust
/// # use stellar_rs::claimable_balances::prelude::*;
/// # use stellar_rs::models::Request;
/// # use stellar_rs::horizon_client::HorizonClient;
/// #
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let base_url = "https://horizon-testnet.stellar.org".to_string();
/// # let horizon_client = HorizonClient::new(base_url)
/// #    .expect("Failed to create Horizon Client");
/// let request = AllClaimableBalancesRequest::new();
///
/// let response = horizon_client
///     .get_all_claimable_balances(&request)
///     .await?;
///
/// // Process the response
/// # Ok({})
/// # }
/// ```
///
pub mod claimable_balances;

/// Client for calling the Stellar Horizon API
///
/// # Constructing a `HorizonClient`
/// A string containing the base URL for the Horizon API is required to contruct a client.
/// For example, to construct a client for the Horizon API testnet:
/// ```rust
/// use stellar_rs::horizon_client::HorizonClient;
///
/// let base_url = "https://horizon-testnet.stellar.org".to_string();
/// let horizon_client = HorizonClient::new(base_url)
///     .expect("Failed to create Horizon Client");;
/// ```
///
/// # Using the `HorizonClient`
/// The HorizonClient has a function that can be called for each endpoind provided
/// by the Horizon API. For example, it has a [`HorizonClient::get_account_list`](crate::horizon_client::HorizonClient::get_account_list)
/// function, which returns an async future that contains a result, as illustrated below:
/// ```rust
/// # use stellar_rs::assets::prelude::{AllAssetsRequest, AllAssetsResponse};
/// # use stellar_rs::models::Request;
/// # use stellar_rs::horizon_client::HorizonClient;
/// #
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let base_url = "https://horizon-testnet.stellar.org".to_string();
/// # let horizon_client = HorizonClient::new(base_url)
/// #    .expect("Failed to create Horizon Client");;
/// let all_assets_request = AllAssetsRequest::new();
/// let accounts_response = horizon_client
///     .get_all_assets(&all_assets_request)
///     .await?;
/// # Ok({})
/// # }
/// ```
pub mod horizon_client;

/// Provides `Request` and `Response` structs for retrieving ledgers.
///
/// The `ledgers` module in the Stellar Horizon SDK includes structures and methods that facilitate
/// querying ledger data from the Horizon server.
///
/// # Usage
///
/// This module is used to construct requests for ledger-related data and to parse the responses
/// received from the Horizon server. It includes request and response structures for both
/// individual ledger queries and queries for a collection of ledgers.
///
/// # Example
///
/// To use this module, you can create an instance of a request struct, such as `SingleLedgerRequest`
/// or `AllLedgersRequest`, set any desired query parameters, and pass the request to the
/// `HorizonClient`. The client will then execute the request and return the corresponding
/// response struct, like `SingleLedgerResponse` or `AllLedgersResponse`.
///
/// ```rust
/// use stellar_rs::horizon_client::HorizonClient;
/// use stellar_rs::ledgers::prelude::*;
/// use stellar_rs::models::Request;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org".to_string())?;
///
/// // Example: Fetching a single ledger by sequence number
/// let single_ledger_request = SingleLedgerRequest::new().set_sequence(123456)?;
/// let ledger_response = horizon_client.get_single_ledger(&single_ledger_request).await?;
///
/// // Example: Fetching all ledgers
/// let all_ledgers_request = LedgersRequest::new().set_limit(10)?;
/// let ledgers_response = horizon_client.get_all_ledgers(&all_ledgers_request).await?;
///
/// // Process the responses...
/// # Ok(())
/// # }
/// ```
///
pub mod ledgers;

/// Provides `Request` and `Response` structs for retrieving effects.
///
/// The `effects` module in the Stellar Horizon SDK includes structures and methods that facilitate
/// querying effect data from the Horizon server.
///
/// # Usage
///
/// This module is used to construct requests for effect-related data and to parse the responses
/// received from the Horizon server. It includes request and response structures for both
/// individual effect queries and queries for a collection of effects.
///
/// # Example
///
/// To use this module, you can create an instance of a request struct, such as `SingleEffectRequest`
/// or `AllEffectsRequest`, set any desired query parameters, and pass the request to the
/// `HorizonClient`. The client will then execute the request and return the corresponding
/// response struct, like `SingleEffectResponse` or `AllEffectsResponse`.
///
/// ```rust
/// use stellar_rs::horizon_client::HorizonClient;
/// use stellar_rs::effects::prelude::*;
/// use stellar_rs::models::Request;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org".to_string())?;
///
/// // Example: Fetching all effects
/// let all_effects_request = AllEffectsRequest::new().set_limit(10)?;
/// let effects_response = horizon_client.get_all_effects(&all_effects_request).await?;
///
/// // Process the responses...
/// # Ok(())
/// # }
/// ```
///
pub mod effects;

/// Contains core data structures and traits.
///
/// This module is used by the Stellar Rust SDK to interact with the Horizon API.
/// It defines enums, traits, and functions that encapsulate the logic for
/// creating and processing HTTP requests and responses, as well as handling the
/// data involved in these operations.
///
/// The `models` module plays a critical role in abstracting the complexities
/// of the Horizon API, allowing developers to work with high-level Rust constructs
/// instead of raw HTTP requests and JSON responses.
pub mod models;

/// Extension trait for building query parameter strings from a vector of optional values.
///
/// This trait provides a method to construct a query string from a vector of optional
/// values (`Option<T>`). It is designed to be used for generating query parameters in
/// URL construction, where each parameter is only included if it has a value (`Some`).
///
/// # Usage
/// This trait is typically used internally in constructing URLs with query parameters
/// by implementors of the [`Request::get_query_parameters`](crate::models::Request::get_query_parameters)
/// method. It enables a convenient and efficient way to handle optional parameters in
/// a URL query string.
///
trait BuildQueryParametersExt<T> {
    /// Constructs a query string for an HTTP request from the object's properties.
    ///
    /// This method transforms the properties of the implementing object into a URL-encoded query
    /// string.
    ///
    fn build_query_parameters(self) -> String;
}

impl<T: ToString> BuildQueryParametersExt<Option<T>> for Vec<Option<T>> {
    /// # Implementation for `Vec<Option<T>>`
    /// Converts each property to a key-value pair, and concatenates pairs with '&'.
    /// Properties that are `None` are omitted from the string.
    ///
    /// ## Returns
    /// A `String` representing the query parameters of the HTTP request. If there
    /// are no parameters, or all properties are `None`, an empty string is returned.
    ///
    fn build_query_parameters(self) -> String {
        let params = self
            .into_iter()
            // Iterate over each element in the vector.
            .filter_map(|x|
                // Use filter_map to process each Option<T>.
                // If the element is Some, it's transformed to its string representation.
                // If the element is None, it's filtered out.
                x.map(|val| val.to_string()))
            // Collect the transformed values into a Vec<String>.
            .collect::<Vec<String>>()
            // Join the Vec<String> elements with '&' to create the query string.
            .join("&");

        // Check if the resulting params string is empty.
        match params.is_empty() {
            // If params is empty, return an empty string.
            true => "".to_string(),
            // If params is not empty, prepend a '?' to the params string.
            false => format!("?{}", params),
        }
    }
}
