/// Provides the `AccountsRequest`.
///
/// This module provides the `AccountsRequest` struct, which is specifically designed 
/// for constructing requests to query for multiple accounts from the Horizon server. It is
/// is meant to be used with the [`HorizonClient::get_accounts_list`](crate::horizon_client::HorizonClient::get_account_list)
/// method.
/// 
/// The detailed documentation and usage instructions for the `AccountsRequest` struct 
/// can be found within the struct definition itself.
///
/// # Usage
///
/// Typically, an instance of `AccountsRequest` is created and potentially configured 
/// with specific query parameters, before being passed to [`HorizonClient::get_accounts_list`](crate::horizon_client::HorizonClient::get_account_list).
/// For example, construct a request to filter by signer:
///
/// ```rust
/// # use stellar_rust_sdk::accounts::accounts_request::AccountsRequest;
/// # use crate::stellar_rust_sdk::models::Request;
/// let mut request = AccountsRequest::new();
/// request
///     .set_signer("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7")
///     .set_limit(10);
/// // Use with HorizonClient::get_account_list
/// ```
///
pub mod accounts_request;

/// Provides the `AccountsResponse`.
///
/// This module provides a set of structures that represent the response received from the Horizon
/// API when querying for accounts. These structures are designed to translate the JSON response
/// from the Horizon server into proper Rust objects, facilitating the processing and utilization
/// of account data within the client application.
/// For a more elaborate overview of the returned values, please refer to the documentation of the 
/// <a href="https://developers.stellar.org/api/horizon/resources/list-all-accounts">List All Accounts endpoint</a>.
///
/// Each structure is equipped with the necessary serialization and deserialization capabilities to handle
/// the JSON data returned by the Horizon server. Additionally, the `Getters` derive macro is used to 
/// provide convenient getter methods for accessing the fields of these structures.
///
/// # Usage
///
/// These structures are mainly used internally by the `HorizonClient` when processing responses from 
/// account-related API calls. The [`AccountsResponse`](crate::accounts::accounts_response::AccountsResponse) struct is of special importance here, 
/// as it is returned by the [`HorizonClient::get_accounts_list`](crate::horizon_client::HorizonClient::get_account_list) method, 
/// providing a user-friendly way to access account data.
///
/// # Example
///
/// ```
/// # use stellar_rust_sdk::accounts::prelude::{AccountsRequest, AccountsResponse};
/// # use stellar_rust_sdk::models::*;
/// # use stellar_rust_sdk::horizon_client::HorizonClient;
/// # 
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let base_url = "https://horizon-testnet.stellar.org".to_string();
/// # let horizon_client = HorizonClient::new(base_url)
/// #    .expect("Failed to create Horizon Client");
/// #
/// # let mut accounts_request = AccountsRequest::new();
/// # accounts_request
/// #     .set_signer("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7")
/// #     .set_limit(10);
/// # 
/// let accounts_response: Result<AccountsResponse, String> = horizon_client
///     .get_account_list(&accounts_request)
///     .await;
/// 
/// // Access the account details
/// for record in accounts_response?._embedded().records() {
///     println!("Account ID: {}", record.account_id());
///     // Further processing...
///  }
/// # Ok({})
/// # }
/// ```
///
pub mod accounts_response;

/// Provides the `SingleAccountRequest`.
///
/// This module provides the `SingleAccountRequest` struct, specifically designed for 
/// constructing requests to query information about a single account from the Horizon 
/// server. It is tailored for use with the [`HorizonClient::get_single_account`](crate::horizon_client::HorizonClient::get_single_account) 
/// method.
///
/// # Usage
///
/// Create an instance of `SingleAccountRequest` and set the account ID to specify the 
/// target account for the query. The struct can then be passed to the `get_single_account` 
/// method of the `HorizonClient` to initiate the request.
///
/// ```
/// # use stellar_rust_sdk::accounts::prelude::SingleAccountRequest;
/// # use crate::stellar_rust_sdk::models::Request;
/// #
/// let mut single_account_request = SingleAccountRequest::new();
/// single_account_request
///     .set_account_id("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string());
/// // Use with HorizonClient::get_single_account
/// ```
///
pub mod single_account_request;

/// Provides the `SingleAccountResponse`.
///
/// This module defines structures representing the response from the Horizon API when querying 
/// for a single account. The structures are designed to deserialize the JSON response into Rust 
/// objects, enabling straightforward access to various details of a single Stellar account.
/// 
/// These structures are equipped with serialization capabilities to handle the JSON data from the 
/// Horizon server and with getter methods for easy field access.
///
/// # Usage
///
/// Primarily used internally by the `HorizonClient` when handling responses from single account
/// queries. The [`SingleAccountResponse`](crate::accounts::single_account_response::SingleAccountResponse) struct, 
/// specifically, is utilized to encapsulate and provide access to the details of the account being queried.
///
/// # Example
/// 
/// ```
/// # use stellar_rust_sdk::accounts::prelude::{SingleAccountRequest, SingleAccountResponse};
/// # use stellar_rust_sdk::models::*;
/// # use stellar_rust_sdk::horizon_client::HorizonClient;
/// # 
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let base_url = "https://horizon-testnet.stellar.org".to_string();
/// # let horizon_client = HorizonClient::new(base_url)
/// #    .expect("Failed to create Horizon Client");
/// #
/// # let mut single_account_request = SingleAccountRequest::new();
/// # single_account_request
/// #     .set_account_id("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string());
/// # 
/// let _single_account_response = horizon_client
///   .get_single_account(&single_account_request)
///   .await;
/// 
/// // Access the account details
/// println!("Account ID: {}", _single_account_response?.account_id());
/// # Ok({})
/// # }
/// ```
///
pub mod single_account_response;

static ACCOUNTS_PATH: &str = "accounts";


pub mod prelude {
    pub use super::accounts_request::*;
    pub use super::accounts_response::*;
    pub use super::single_account_request::*;
    pub use super::single_account_response::*;
}
