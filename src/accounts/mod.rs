use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{Flags, SelfLink, TemplateLink};

/// Provides the `AccountsRequest`.
///
/// This module provides the `AccountsRequest` struct, which is specifically designed
/// for constructing requests to query for multiple accounts from the Horizon server. It is
/// is meant to be used with the [`HorizonClient::get_accounts_list`](crate::horizon_client::HorizonClient::get_account_list)
/// method.
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
/// These structures are mainly used internally by the `HorizonClient` when processing responses from
/// account-related API calls. The [`AccountsResponse`](crate::accounts::accounts_response::AccountsResponse) struct is of special importance here,
/// as it is returned by the [`HorizonClient::get_accounts_list`](crate::horizon_client::HorizonClient::get_account_list) method,
/// providing a user-friendly way to access account data.
///
pub mod accounts_response;

/// Provides the `SingleAccountRequest`.
///
/// This module provides the `SingleAccountRequest` struct, specifically designed for
/// constructing requests to query information about a single account from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_single_account`](crate::horizon_client::HorizonClient::get_single_account)
/// method.
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
pub mod single_account_response;

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
    pub use super::accounts_response::*;
    pub use super::single_account_request::*;
    pub use super::single_account_response::*;
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct AccountRecord {
    /// Links associated with this account record.
    #[serde(rename = "_links")]
    links: AccountResponseLinks,
    /// The unique identifier of the account.
    id: String,
    /// The public key of the account.
    account_id: String,
    /// The sequence number of the account.
    sequence: String,
    /// The number of subentries in the account.
    subentry_count: i32,
    last_modified_ledger: u64,
    last_modified_time: String,
    /// The thresholds for different operations in the account.
    thresholds: Thresholds,
    /// The flags set on the account.
    flags: Flags,
    /// A list of balances for different assets held by the account.
    balances: Vec<Balances>,
    /// A list of signers associated with the account.
    signers: Vec<Signer>,
    /// Additional data associated with the account (in JSON format).
    data: serde_json::Value,
    /// The number of entries the account is sponsoring.
    num_sponsoring: i32,
    /// The number of entries the account is sponsored for.
    num_sponsored: i32,
    /// A token used for paging through results.
    paging_token: String,
}

/// Represents the operational thresholds for a single account.
///
/// This struct defines the low, medium, and high thresholds for operations on an account, 
/// determining the minimum level of authorization required for various types of transactions.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Thresholds {
    /// The low threshold value for operations.
    low_threshold: u32,
    /// The medium threshold value for operations.
    med_threshold: u32,
    /// The high threshold value for operations.
    high_threshold: u32,
}

/// Represents a single signer within a single account.
///
/// This struct details information about a signer for an account, including their key, 
/// weight in authorization decisions, and type.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Signer {
    /// The weight of the signer's vote in authorization decisions.
    weight: u32,
    /// The key associated with the signer.
    key: String,
    /// The type of the signer (e.g., 'ed25519_public_key').
    #[serde(rename = "type")]
    singer_type: String,
}

/// Represents a single balance within a single account.
///
/// This struct encapsulates the details of a single balance, including the amount, liabilities, 
/// and the type of the asset.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Balances {
    /// The total balance of the asset.
    balance: String,
    /// Buying liabilities associated with the asset.
    buying_liabilities: String,
    /// Selling liabilities associated with the asset.
    selling_liabilities: String,
    /// The type of the asset (e.g., native, credit_alphanum4, credit_alphanum12).
    asset_type: String,
}

/// Represents the navigational links in a single account response from the Horizon API.
///
/// This struct includes various hyperlinks such as links to the account itself, transactions,
/// operations, payments, effects, offers, trades, and data, providing quick access to related resources.
///
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct AccountResponseLinks {
    /// The link to the account itself.
    #[serde(rename = "self")]
    self_link: SelfLink,
    /// Link to the account's transactions.
    transactions: TemplateLink,
    /// Link to the account's operations.
    operations: TemplateLink,
    /// Link to the account's payments.
    payments: TemplateLink,
    /// Link to the effects concerning the account.
    effects: TemplateLink,
    /// Link to the account's offers.
    offers: TemplateLink,
    /// Link to the trades involving the account.
    trades: TemplateLink,
    /// Link to the account's additional data.
    data: TemplateLink,
}
