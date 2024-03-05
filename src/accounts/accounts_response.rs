use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{models::Response, Embedded, Flags, ResponseLinks};

use super::AccountResponseLinks;

/// Represents the entire response for the list all accounts query.
///
/// This struct defines the overall structure of the response returned from the Horizon API
/// when querying for multiple accounts. It includes navigational links and the embedded data
/// containing account records.
///
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct AccountsResponse {
    /// Navigational links related to the response.
    _links: ResponseLinks,
    /// The embedded object containing the actual account records.
    _embedded: Embedded<Record>,
}

/// Represents a single balance within an account in the list of all accounts response.
///
/// This struct details the balance information for an account, including the type of asset
/// and the associated liabilities.
///
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Balances {
    /// The total balance of the asset.
    balance: String,
    /// Buying liabilities associated with the asset.
    buying_liabilities: String,
    /// Selling liabilities associated with the asset.
    selling_liabilities: String,
    /// The type of asset (e.g., native, credit_alphanum4, credit_alphanum12).
    asset_type: String,
}

/// Represents the thresholds associated with an account in the list of all accounts response.
///
/// This struct encapsulates the various threshold values for different operations in an account.
///
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct AccountsResponseThresholds {
    /// The threshold for low-level operations.
    low_threshold: i32,
    /// The threshold for medium-level operations.
    med_threshold: i32,
    /// The threshold for high-level operations.
    high_threshold: i32,
}

/// Represents a signer associated with an account in the list of all accounts response.
///
/// This struct details the information about a signer for an account, including their
/// key, type, and weight in authorization decisions.
///
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Signers {
    /// The weight of the signer's vote in authorization decisions.
    weight: i32,
    /// The key associated with the signer.
    key: String,
    /// The type of the signer (e.g., 'ed25519_public_key').
    #[serde(rename = "type")]
    signer_type: String,
}

/// Represents a single account record in the list of all accounts response.
///
/// This struct encapsulates detailed information about an individual account as returned
/// in the Horizon API response. It includes various fields like account identifiers,
/// thresholds, flags, balances, and more.
///
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Record {
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
    thresholds: AccountsResponseThresholds,
    /// The flags set on the account.
    flags: Flags,
    /// A list of balances for different assets held by the account.
    balances: Vec<Balances>,
    /// A list of signers associated with the account.
    signers: Vec<Signers>,
    /// Additional data associated with the account (in JSON format).
    data: serde_json::Value,
    /// The number of entries the account is sponsoring.
    num_sponsoring: i32,
    /// The number of entries the account is sponsored for.
    num_sponsored: i32,
    /// A token used for paging through results.
    paging_token: String,
}

impl Response for AccountsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}