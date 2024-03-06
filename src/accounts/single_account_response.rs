extern crate serde;
extern crate serde_json;

use derive_getters::Getters;
use serde::Deserialize;

use crate::{models::Response, Flags};

use super::{AccountResponseLinks, Balances, Signer, Thresholds};

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
    _links: AccountResponseLinks,
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

impl Response for SingleAccountResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}