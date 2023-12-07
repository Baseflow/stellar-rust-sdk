extern crate serde;
extern crate serde_json;

use derive_getters::Getters;
use serde::Deserialize;

use crate::models::Response;

/// Represents the navigational links in a single account response from the Horizon API.
///
/// This struct includes various hyperlinks such as links to the account itself, transactions,
/// operations, payments, effects, offers, trades, and data, providing quick access to related resources.
///
/// # Fields
///
/// * `operations`: Link to the account's operations.
/// * `payments`: Link to the account's payments.
/// * `effects`: Link to the effects concerning the account.
/// * `offers`: Link to the account's offers.
/// * `trades`: Link to the trades involving the account.
/// * `data`: Link to the account's additional data.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Links {
    /// The link to the account itself.
    #[serde(rename = "self")]
    self_link: SelfLink,
    /// Link to the account's transactions.
    transactions: Link,
    operations: Link,
    payments: Link,
    effects: Link,
    offers: Link,
    trades: Link,
    data: Link,
}

/// Represents the self-link in the list of all accounts response.
///
/// This struct defines the structure of the self-link (`href`) found in the accounts response
/// from the Horizon API. It contains the URL to the current resource or query.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SelfLink {
    /// A `String` representing the hyperlink reference to the current resource or query.
    href: String,
}

/// Struct defining a single balance in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Link {
    href: String,
    templated: bool,
}

/// Struct defining the thresholds in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Thresholds {
    low_threshold: u32,
    med_threshold: u32,
    high_threshold: u32,
}

/// Struct defining the flags in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Flags {
    auth_required: bool,
    auth_revocable: bool,
    auth_immutable: bool,
    auth_clawback_enabled: bool,
}

/// Struct defining a single signer in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Balance {
    balance: String,
    buying_liabilities: String,
    selling_liabilities: String,
    asset_type: String,
}

/// Struct defining a single signer in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Signer {
    weight: u32,
    key: String,
    #[serde(rename = "type")]
    type_: String,
}

/// Struct defining a record of a single account in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Data {}

/// Struct defining a record of a single account in the list all accounts response.
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SingleAccountsResponse {
    _links: Links,
    id: String,
    account_id: String,
    sequence: String,
    subentry_count: u32,
    last_modified_ledger: u64,
    last_modified_time: String,
    thresholds: Thresholds,
    flags: Flags,
    balances: Vec<Balance>,
    signers: Vec<Signer>,
    data: Option<Data>,
    num_sponsoring: u32,
    num_sponsored: u32,
    paging_token: String,
}

impl Response for SingleAccountsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}