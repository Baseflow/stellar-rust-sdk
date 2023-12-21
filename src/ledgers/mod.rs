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
    pub use super::single_ledger_response::*;
}
