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
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Links {
    /// The link to the account itself.
    #[serde(rename = "self")]
    self_link: SelfLink,
    /// Link to the account's transactions.
    transactions: Link,
    /// Link to the account's operations.
    operations: Link,
    /// Link to the account's payments.
    payments: Link,
    /// Link to the effects concerning the account.
    effects: Link,
    /// Link to the account's offers.
    offers: Link,
    /// Link to the trades involving the account.
    trades: Link,
    /// Link to the account's additional data.
    data: Link,
}

/// Represents the self-link in the list of single account response.
///
/// This struct defines the structure of the self-link (`href`) found in the accounts response
/// from the Horizon API. It contains the URL to the current resource or query.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SelfLink {
    /// A `String` representing the hyperlink reference to the current resource or query.
    href: String,
}

/// Represents a single hyperlink with a templating option.
///
/// This struct is used for individual links within the account's response, indicating the 
/// URL of the resource and whether it supports templating.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Link {
    /// The URL of the linked resource.
    href: String,
    /// A boolean indicating if the link is templated.
    templated: bool,
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

/// Represents the authorization flags set on a single account.
///
/// This struct defines the various boolean flags that can be set on an account, 
/// indicating specific permissions or settings.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Flags {
    /// Indicates if authorization is required for transactions.
    auth_required: bool,
    /// Indicates if authorization can be revoked.
    auth_revocable: bool,
    /// Indicates if the account's authorization settings cannot be changed.
    auth_immutable: bool,
    /// Indicates if the clawback feature is enabled.
    auth_clawback_enabled: bool,
}


/// Represents a single balance within a single account.
///
/// This struct encapsulates the details of a single balance, including the amount, liabilities, 
/// and the type of the asset.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Balance {
    /// * `balance`: The total balance of the asset.
    balance: String,
    /// * `buying_liabilities`: Buying liabilities associated with the asset.
    buying_liabilities: String,
    /// * `selling_liabilities`: Selling liabilities associated with the asset.
    selling_liabilities: String,
    /// * `asset_type`: The type of the asset (e.g., native, credit_alphanum4, credit_alphanum12).
    asset_type: String,
}

/// Represents a single signer within a single account.
///
/// This struct details information about a signer for an account, including their key, 
/// weight in authorization decisions, and type.
///
/// # Fields
///
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Signer {
    /// The weight of the signer's vote in authorization decisions.
    weight: u32,
    /// The key associated with the signer.
    key: String,
    /// The type of the signer (e.g., 'ed25519_public_key').
    #[serde(rename = "type")]
    type_: String,
}

/// Represents additional data associated with a single account in the Horizon API response.
///
/// This struct is intended to encapsulate any extra data fields that may be included in the account's response.
/// In its current form, it acts as a placeholder for potential future expansions of the account data model in the
/// Horizon API.
///
/// # Note
///
/// As of now, `Data` does not contain any fields. However, it is included in the account structure to accommodate
/// additional data that may be added to the Horizon API response in the future. It can be extended to include specific
/// fields as needed.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Data {
    // Future fields to be added here
}

/// Represents the response for a single account query in the Horizon API.
///
/// This struct defines the overall structure of the response for a single account query. 
/// It includes navigational links, account identifiers, thresholds, flags, balances, 
/// signers, and additional data related to the account.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SingleAccountResponse {
    /// Navigational links related to the account.
    _links: Links,
    /// The unique identifier of the account.
    id: String,
    /// The public key of the account.
    account_id: String,
    /// The sequence number of the account.
    sequence: String,
    /// The number of subentries in the account.
    subentry_count: u32,
    /// The ledger number in which the account was last modified.
    last_modified_ledger: u64,
    /// The time at which the account was last modified.
    last_modified_time: String,
    /// The operational thresholds for the account.
    thresholds: Thresholds,
    /// The flags set on the account.
    flags: Flags,
    /// A list of balances for different assets held by the account.
    balances: Vec<Balance>,
    /// A list of signers associated with the account.
    signers: Vec<Signer>,
    /// Additional data associated with the account.
    data: Option<Data>,
    /// The number of entries the account is sponsoring.
    num_sponsoring: u32,
    /// The number of entries the account is sponsored for.
    num_sponsored: u32,
    /// A token used for paging through results.
    paging_token: String,
}

impl Response for SingleAccountResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}