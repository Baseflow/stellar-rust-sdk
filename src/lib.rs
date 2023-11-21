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
//! use stellar_rust_sdk::horizon_client::horizon_client::HorizonClient;
//! use stellar_rust_sdk::accounts::prelude::AccountsRequest;
//! use stellar_rust_sdk::accounts::prelude::AccountsResponse;
//! use crate::stellar_rust_sdk::models::Request;
//! 
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize horizon client
//! let horizon_client =
//!     HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();
//!
//! // construct request
//! let mut accounts_request = AccountsRequest::new()
//!     .set_signer("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7").unwrap()
//!     .set_limit(10).unwrap();
//!
//! // call the get_account_list method to retrieve the account list response
//! let _accounts_response: Result<AccountsResponse, String> =
//!     horizon_client.get_account_list(&accounts_request).await;
//!
//! assert!(_accounts_response.is_ok());
//!     Ok(())
//! }
//! ```
//! ## Features
//!
//! All the endpoints in the Horizon Api and wether or not they are supported by this library:
//!
//! - [x] `Accounts` - List all accounts endpoint
//! - [x] `Single Account` - Get a single account endpoint
//! - [x] `All Assets` - List all assets endpoint
//! - [x] `All Claimable Balances` - List all claimable balances endpoint
//! - [x] `Single Claimable Balance` - Get a single claimable balance endpoint
//! - [ ] `All Effects` - List all effects endpoint
//! - [ ] `Effects for Account` - List all effects for an account endpoint`
//! - [ ] `Effects for Ledger` - List all effects for a ledger endpoint
//! - [ ] `Effects for Operation` - List all effects for an operation endpoint
//! - [ ] `Effects for Transaction` - List all effects for a transaction endpoint
//! - [ ] `All Fee Stats` - List all fee stats endpoint
//! - [x] `All Ledgers` - List all ledgers endpoint
//! - [x] `Single Ledgers` - Get a single ledger endpoint
//! - [ ] `All Liquidity Pools` - List all liquidity pools endpoint
//! - [ ] `Single Liquidity Pool` - Get a single liquidity pool endpoint
//! - [ ] `All Offers` - List all offers endpoint
//! - [ ] `Single Offer` - Get a single offer endpoint
//! - [ ] `Offers for Account` - List all offers for an account endpoint
//! - [ ] `All Operations` - List all operations endpoint
//! - [ ] `Single Operation` - Get a single operation endpoint
//! - [ ] `Operations for Account` - List all operations for an account endpoint
//! - [ ] `Operations for Ledger` - List all operations for a ledger endpoint
//! - [ ] `Operations for Liquidity Pool` - List all operations for a liquidity pool endpoint
//! - [ ] `Operations for Transaction` - List all operations for a transaction endpoint
//! - [ ] `All Order Book Details` - List all order book endpoint
//! - [ ] `Find payments Paths` - Find payment paths endpoint
//! - [ ] `Find Strict Receive Payment Paths` - Find strict receive payment paths endpoint
//! - [ ] `Find Strict Send Payment Paths` - Find strict send payment paths endpoint
//! - [ ] `All Payments` - List all payments endpoint
//! - [ ] `Payments for Account` - List all payments for an account endpoint
//! - [ ] `Payments for Ledger` - List all payments for a ledger endpoint
//! - [ ] `Payments for Transaction` - List all payments for a transaction endpoint
//! - [ ] `Trade Aggregations` - List trade aggregations endpoint
//! - [ ] `All Trades` - List all trades endpoint
//! - [ ] `Trades for Account` - List all trades for an account endpoint
//! - [ ] `Trades for Liquidity Pool` - List all trades for a liquidity pool endpoint
//! - [ ] `Trades for Offer` - List all trades for an offer endpoint
//! - [ ] `All Transactions` - List all transactions endpoint
//! - [ ] `Single Transaction` - Get a single transaction endpoint
//! - [ ] `Post Transaction` - Post transaction endpoint
//! - [ ] `Transactions for Account` - List all transactions for an account endpoint
//! - [ ] `Transactions for Ledger` - List all transactions for a ledger endpoint
//! - [ ] `Transactions for Liquidity Pool` - List all transactions for a liquidity pool endpoint

/// The accounts module
pub mod accounts;
/// The assets module
pub mod assets;
/// The claimable balances module
pub mod claimable_balances;
/// The horizon client module
pub mod horizon_client;
/// The ledgers module
pub mod ledgers;
/// The models module, here the Request and Response traits are defined
pub mod models;
pub mod xdr;

/// The asset type
/// Native - The native asset
/// Issued - An issued asset
#[derive(Clone)]
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
