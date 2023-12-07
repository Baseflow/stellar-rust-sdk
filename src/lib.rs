//! Stellar SDK for Rust
//!
//! This is a Rust SDK for the Stellar Horizon API. It is a work in progress and is not yet ready for production use.
//! This library defines traits for request and response objects. This library contains types that implement these traits for each endpoint in the Horizon API respectively.
//! The request objects are used to build the request URL and the response objects are used to deserialize the response from JSON.
//!
//! ## Examples
//!
//! Here is an example on how to use this library:
//!
//! ```rust
//!
//! use stellar_rust_sdk::horizon_client::HorizonClient;
//! use stellar_rust_sdk::accounts::prelude::AccountsRequest;
//! use stellar_rust_sdk::accounts::prelude::AccountsResponse;
//! use crate::stellar_rust_sdk::models::Request;
//! 
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize horizon client
//!     let horizon_client =
//!         HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();
//! 
//!     // construct request
//!     let mut accounts_request = AccountsRequest::new();
//!     accounts_request
//!         .set_signer("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7")
//!         .set_limit(10);
//! 
//!     // call the get_account_list method to retrieve the account list response
//!     let _accounts_response: Result<AccountsResponse, String> =
//!         horizon_client.get_account_list(&accounts_request).await;
//! 
//!     assert!(_accounts_response.is_ok());
//!     Ok(())
//! }
//! ```
//! 
//! ## Implementation Progress
//! ![25%](https://progress-bar.dev/25/?width=200)
//! 
//! This SDK is under active development so expect more and more supported [Horizon API endpoints](https://laboratory.stellar.org/#explorer?network=test)!
//! 
//! #### Supported endpoints:
//! * Accounts
//! * Assets
//! * Claimable balance
//! * Ledgers
//! 
//! #### Endpoints on the roadmap:
//! * Effects
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

/// Requests and Response for `accounts`.
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
/// # use stellar_rust_sdk::accounts::prelude::{AccountsRequest, AccountsResponse};
/// # use stellar_rust_sdk::models::Request;
/// # use stellar_rust_sdk::horizon_client::HorizonClient;
/// # 
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let base_url = "https://horizon-testnet.stellar.org".to_string();
/// # let horizon_client = HorizonClient::new(base_url)
/// #    .expect("Failed to create Horizon Client");;
/// let mut request = AccountsRequest::new();
/// request
///     .set_signer("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7")
///     .set_limit(10);
/// 
/// let response: Result<AccountsResponse, String> = horizon_client
///     .get_account_list(&request)
///     .await;
/// # Ok({})
/// # }
/// 
pub mod accounts;


/// The assets module
pub mod assets;


/// The claimable balances module
pub mod claimable_balances;

/// Client for calling the Stellar Horizon API 
/// 
/// # Constructing a `HorizonClient`
/// A string containing the base URL for the Horizon API is required to contruct a client.
/// For example, to construct a client for the Horizon API testnet:
/// ```rust
/// use stellar_rust_sdk::horizon_client::HorizonClient;
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
/// # use stellar_rust_sdk::assets::prelude::{AllAssetsRequest, AllAssetsResponse};
/// # use stellar_rust_sdk::models::Request;
/// # use stellar_rust_sdk::horizon_client::HorizonClient;
/// # 
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let base_url = "https://horizon-testnet.stellar.org".to_string();
/// # let horizon_client = HorizonClient::new(base_url)
/// #    .expect("Failed to create Horizon Client");;
/// let all_assets_request = AllAssetsRequest::new();
/// let accounts_response: Result<AllAssetsResponse, String> = horizon_client
///     .get_all_assets(&all_assets_request)
///     .await;
/// # Ok({})
/// # }
/// ```
pub mod horizon_client;

/// The ledgers module
pub mod ledgers;
/// The models module, here the Request and Response traits are defined
pub mod models;

mod xdr;

/// The asset type
/// Native - The native asset
/// Issued - An issued asset
pub enum AssetType {
    Native,
    Issued,
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AssetType::Native => write!(f, "native"),
            AssetType::Issued => write!(f, "issued"),
        }
    }
}

/// The order of the records
/// Asc - Ascending order
/// Desc - Descending order
pub enum Order {
    Asc,
    Desc,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Order::Asc => write!(f, "asc"),
            Order::Desc => write!(f, "desc"),
        }
    }
}

trait BuildQueryParametersExt<T> {
    fn build_query_parameters(self) -> String;
}

impl<T: ToString> BuildQueryParametersExt<Option<T>> for Vec<Option<T>> {
    fn build_query_parameters(self) -> String {
        let params = self.into_iter()
            // The filter_map function filters out the None values, leaving only the Some values with formatted strings.
            .filter_map(|x| x.map(|val| val.to_string()))
            .collect::<Vec<String>>().join("&");
        match params.is_empty() {
            true => "".to_string(),
            false => format!("?{}", params),
        }
    }
}
