use crate::models::prelude::*;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// Represents the navigational links in a response from the Horizon API.
///
/// # Usage
/// This struct includes various hyperlinks such as links to the transaction itself and
/// the ledger that the transaction was included in.
///
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct TransactionResponseLinks {
    #[serde(rename = "self")]
    self_link: Link,
    account: Link,
    ledger: Link,
    operations: Link,
    effects: Link,
    precedes: Link,
    succeeds: Link,
    transaction: Link,
}

/// Represents the set of transaction preconditions affecting its validity.
///
/// # Usage
/// This struct details information about the preconditions, including the time bounds, ledger bounds (optional),
/// minimum account sequence and its age(optional), mimimum account sequence leder gap (optional,
/// and an array of up to 2 additional signers (optional).
/// 
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
pub struct Preconditions {
    /// The time range for which this transaction is valid, with bounds as unsigned 64-bit UNIX timestamps.
    timebounds: TimeBounds,
    // The ledger range for which this transaction is valid.
    ledger_bounds: Option<LedgerBounds>,
    /// Containing a positive, signed 64-bit integer representing the lowest source account sequence number for which the transaction is valid.
    min_account_sequence: Option<String>,
    /// The minimum duration of time (in seconds as an unsigned 64-bit integer) that must have passed since the source account's sequence number changed for the transaction to be valid.
    min_account_sequence_age: Option<i64>,
    /// An unsigned 32-bit integer representing the minimum number of ledgers that must have closed since the source account's sequence number changed for the transaction to be valid.
    min_account_sequence_ledger_gap: Option<i64>,
    /// The list of up to two additional signers that must have corresponding signatures for this transaction to be valid.
    extra_signers: Option<Vec<String>>,
}

/// Represents the time range for which this transaction is valid, with bounds as unsigned 64-bit UNIX timestamps.
///
/// # Usage
/// This struct details information about the time range, including the lower time bound 
/// and the upper time bound (optional).
/// 
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
pub struct TimeBounds {
    /// The lower bound.
    min_time: String,
    /// The upper bound.
    max_time: Option<String>,
}

/// Represents the the ledger range for which this transaction is valid.
///
/// # Usage
/// This struct details information about the ledger range, including the lower ledger bound 
/// and the upper ledger bound (optional).
/// 
#[derive(Default, Debug, Clone, Serialize, Deserialize, Getters)]
pub struct LedgerBounds {
    /// The lower bound.
    min_ledger: String,
    /// The upper bound.
    max_ledger: Option<String>,
}

/// Represents the response for the 'all transactions' query in the Horizon API.
///
/// This struct defines the overall structure of the response for an 'all transactions' query.
/// It includes navigational links and embedded results.
///
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AllTransactionsResponse {
    #[serde(rename = "_links")]
    links: ResponseLinks,
    #[serde(rename = "_embedded")]
    embedded: Embedded<TransactionResponse>,
}

impl Response for AllTransactionsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}

/// Represents a single transaction record in the Horizon API response.
///
/// # Usage
/// This struct encapsulates detailed information about a single transaction, including its ID,
/// hash, creation time, source account, and other relevant data.
/// 
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct TransactionResponse {
        #[serde(rename = "_links")]
        links: TransactionResponseLinks,
        /// A unique identifier for this transaction.
        id: String,
        /// A cursor value for use in pagination.
        paging_token: String,
        /// Indicates if this transaction was successful or not.
        successful: bool,
        /// A hex-encoded SHA-256 hash of this transaction’s XDR-encoded form.
        hash: String,
        /// The sequence number of the ledger that this transaction was included in.
        ledger: i64,
        /// The date this transaction was created.
        created_at: String,
        /// The account that originates the transaction.
        source_account: String,
        // TODO: Missing description in Stellar documentation.
        account_muxed: Option<String>,
        // TODO: Missing description in Stellar documentation.
        account_muxed_id: Option<String>,
        /// The source account's sequence number that this transaction consumed.
        source_account_sequence: String,
        /// The ID of the fee account.
        fee_account: String,
        /// The fee (in stroops) paid by the source account to apply this transaction to the ledger.
        fee_charged: String,
        /// The maximum fee (in stroops) that the source account was willing to pay.
        max_fee: String,
        /// The number of operations contained within this transaction.
        operation_count: i64,
        /// A base64 encoded string of the raw `TransactionEnvelope` XDR struct for this transaction.
        envelope_xdr: String,
        /// A base64 encoded string of the raw `TransactionResult` XDR struct for this transaction.
        result_xdr: String,
        /// A base64 encoded string of the raw `TransactionMeta` XDR struct for this transaction
        result_meta_xdr: String,
        /// A base64 encoded string of the raw `L`edgerEntryChanges` XDR struct produced by taking fees for this transaction.
        fee_meta_xdr: String,
        /// The optional memo attached to a transaction.
        memo: Option<String>,
        /// The type of memo. Potential values include `MEMO_TEXT`, `MEMO_ID`, `MEMO_HASH`, `MEMO_RETURN`.
        memo_type: String,
        /// An array of signatures used to sign this transaction.
        signatures: Vec<String>,
        /// The date after which a transaction is valid. 
        valid_after: Option<String>,
        /// The date before which a transaction is valid.
        valid_before: Option<String>,
        /// A set of transaction preconditions affecting its validity.
        preconditions: Option<Preconditions>,
}

impl Response for TransactionResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}