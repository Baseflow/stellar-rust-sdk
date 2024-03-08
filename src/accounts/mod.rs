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
///
static ACCOUNTS_PATH: &str = "accounts";

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
