use crate::models::prelude::*;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// Represents the entire response for the list all accounts query.
///
/// This struct defines the overall structure of the response returned from the Horizon API
/// when querying for multiple accounts. It includes navigational links and the embedded data
/// containing account records.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct AccountsResponse {
    /// Navigational links related to the response.
    #[serde(rename = "_links")]
    links: ResponseLinks,
    /// The embedded object containing the actual account records.
    #[serde(rename = "_embedded")]
    embedded: Embedded<Account>,
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
    self_link: Link,
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

/// Represents the response for a single account query in the Horizon API.
///
/// This struct defines the overall structure of the response for a single account query.
/// It includes navigational links, account identifiers, thresholds, flags, balances,
/// signers, and additional data related to the account.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Account {
    /// Navigational links related to the account.
    #[serde(rename = "_links")]
    links: AccountResponseLinks,
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
    balances: Vec<Balances>,
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

impl Response for Account {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

impl Response for AccountsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}
