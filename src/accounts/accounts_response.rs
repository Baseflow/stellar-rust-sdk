use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{models::Response, Embedded, Flags, ResponseLinks};

use super::{AccountResponseLinks, Balances, Signer, Thresholds};

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
    embedded: Embedded<Record>,
}

/// Represents a single account record in the list of all accounts response.
///
/// This struct encapsulates detailed information about an individual account as returned
/// in the Horizon API response. It includes various fields like account identifiers,
/// thresholds, flags, balances, and more.
///
#[derive(Debug, Deserialize, Clone, Getters)]
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

impl Response for AccountsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}