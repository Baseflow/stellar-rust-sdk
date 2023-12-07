/// Provides `AccountsRequest` struct to use with `HorizonClient`.
///
/// This module provides the `AccountsRequest` struct, which is specifically designed 
/// for constructing requests to query for multiple accounts from the Horizon server. It is
/// is meant to be used with the [`HorizonClient::get_accounts_list`](crate::HorizonClient::get_account_list)
/// method.
/// 
/// The detailed documentation and usage instructions for the `AccountsRequest` struct 
/// can be found within the struct definition itself.
///
/// # Usage
///
/// Typically, an instance of `AccountsRequest` is created and potentially configured 
/// with specific query parameters, before being passed to [`HorizonClient::get_accounts_list`](crate::HorizonClient::get_account_list).
/// For example, construct a request to filter by signer:
///
/// ```
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

pub mod accounts_response;
pub mod single_account_request;
pub mod single_account_response;

static ACCOUNTS_PATH: &str = "accounts";


pub mod prelude {
    pub use super::accounts_request::*;
    pub use super::accounts_response::*;
    pub use super::single_account_request::*;
    pub use super::single_account_response::*;
}
