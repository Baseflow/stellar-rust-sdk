use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{SelfLink, TemplateLink};

/// Provides the `LedgersRequest` struct..
///
/// This submodule contains the `LedgersRequest` struct, which is designed to create requests
/// for retrieving ledger data from the Stellar network via the Horizon API. It supports
/// various query parameters to refine the ledger data retrieval, such as specifying
/// a range of ledgers, pagination options, and ordering.
///
/// The `LedgersRequest` struct is intended to be used with the `HorizonClient` for
/// making ledger-related API calls to the Horizon server. It enables fetching detailed
/// information about the ledgers in the Stellar network, including transactions, operations,
/// and effects within each ledger.
///
pub mod ledgers_request;

/// Provides the `LedgersResponse` struct.
///
/// This module contains structures that represent the response received from the Horizon API when
/// querying for ledger data. These structures are designed to parse and encapsulate the JSON response
/// from the Horizon server, making it easier to work with ledger data in Rust applications.
///
/// The primary structure in this module is `LedgersResponse`, which contains detailed information about
/// each ledger, including its transactions, operations, and other related data.
///
pub mod ledgers_response;

/// Provides the `SingleLedgerRequest` struct.
///
/// This module provides the `SingleLedgerRequest` struct, designed for constructing requests
/// to retrieve detailed information about a specific ledger from the Stellar Horizon API.
/// The primary use of this struct is to specify the sequence number of the desired ledger,
/// which uniquely identifies it within the Stellar network.
///
/// The `SingleLedgerRequest` struct is primarily used with the
/// [`HorizonClient::get_single_ledger`](crate::horizon_client::HorizonClient::get_single_ledger) method. This approach
/// allows users to fetch specific ledger details, such as transactions, operations, and more,
/// based on the ledger sequence number.
///
pub mod single_ledger_request;

/// Provides the `SingleLedgerResponse` structure.
///
/// This module contains the `SingleLedgerResponse` struct, which is designed to represent the response
/// received from the Stellar Horizon API when querying for a single ledger. It encapsulates all the
/// details of a ledger, including its transactions, operations, and various attributes.
///
/// The `SingleLedgerResponse` struct is particularly useful for applications that need to access specific
/// details about a single ledger, such as its transaction count, operation count, total coins, and so on.

///
pub mod single_ledger_response;

/// The base path for ledger-related endpoints in the Horizon API.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for ledger-related API calls.
///
static LEDGERS_PATH: &str = "ledgers";

/// The `prelude` module for the `ledgers` module in the Stellar Horizon Rust SDK.
///
/// This module serves as a convenient gateway for importing frequently used items
/// related to ledger data and queries. By aggregating key structs and traits from
/// various submodules under `ledgers`, it simplifies the process of accessing these
/// elements within client applications.
///
/// By importing from `prelude`, users can effortlessly integrate the primary functionalities
/// of the `ledgers` module into their code, reducing the need for individual imports and
/// enhancing code readability.
///
/// # Contents
///
/// The `prelude` module includes the following re-exports:
///
/// * From `ledgers_request`: All items (e.g., `LedgersRequest`).
/// * From `ledgers_response`: All items (e.g., `LedgersResponse`, `Record`, etc.).
/// * From `single_ledger_request`: All items (e.g., `SingleLedgerRequest`).
/// * From `single_ledger_response`: All items (e.g., `SingleLedgerResponse`, `Links`, etc.).
///
/// # Example
///
/// ```rust
/// // Import the contents of the ledgers prelude
/// use stellar_rs::ledgers::prelude::*;
///
/// // This allows for direct usage of LedgersRequest, SingleLedgerResponse, etc.
/// let ledger_request = LedgersRequest::new();
/// // Further usage...
/// ```
///
pub mod prelude {
    pub use super::ledgers_request::*;
    pub use super::ledgers_response::*;
    pub use super::single_ledger_request::*;
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct LedgerRecordLinks {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
    pub transactions: TemplateLink,
    pub operations: TemplateLink,
    pub payments: TemplateLink,
    pub effects: TemplateLink,
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct LedgerRecord {
    /// Navigational links related to the ledger.
    _links: LedgerRecordLinks,
    /// The unique identifier of the ledger.
    id: String,
    /// A token used for paging through results.
    paging_token: String,
    /// The hash of the ledger.
    hash: String,
    /// The hash of the previous ledger.
    prev_hash: String,
    /// The sequence number of the ledger.
    sequence: i32,
    /// The number of successful transactions in the ledger.
    successful_transaction_count: i32,
    /// The number of failed transactions in the ledger.
    failed_transaction_count: i32,
    /// The total number of operations in the ledger.
    operation_count: i32,
    /// The number of operations in the transaction set.
    tx_set_operation_count: i32,
    /// The closing time of the ledger.
    closed_at: String,
    /// The total number of coins in the ledger.
    total_coins: String,
    /// The total fees in the ledger's fee pool.
    fee_pool: String,
    /// The base fee in stroops for transactions in the ledger.
    base_fee_in_stroops: i32,
    /// The base reserve in stroops required for an account in the ledger.
    base_reserve_in_stroops: i32,
    /// The maximum size of a transaction set in the ledger.
    max_tx_set_size: i32,
    /// The protocol version used in the ledger.
    protocol_version: i32,
    /// The XDR-encoded header of the ledger.
    header_xdr: String,
}