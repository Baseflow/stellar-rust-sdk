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
/// # Usage
///
/// To use this module, create an instance of `LedgersRequest` and set any desired pagination
/// options. Then, pass the request object to the appropriate method of `HorizonClient`
/// to execute the query. The client will return a response containing the ledger data.
///
/// # Example
/// ```rust
/// use stellar_rust_sdk::horizon_client::HorizonClient;
/// use stellar_rust_sdk::ledgers::ledgers_request::LedgersRequest;
/// use stellar_rust_sdk::models::*;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org".to_string())?;
///
/// // Constructing a request to fetch ledgers
/// let ledgers_request = LedgersRequest::new()
///     .set_limit(10)? // Setting the number of ledgers to retrieve
///     .set_order(Order::Desc); // Setting the order of ledgers
///
/// // Fetching the ledgers from Horizon
/// let ledgers_response = horizon_client.get_all_ledgers(&ledgers_request).await?;
///
/// // Processing the response...
/// # Ok(())
/// # }
/// ```
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
/// # Usage
/// The `LedgersResponse` and other related structures in this module are typically used internally by the
/// `HorizonClient` when handling responses from ledger-related API calls. However, they can also be used directly
/// in client applications for custom processing of ledger data.
///
/// # Example
/// ```rust 
/// # use stellar_rust_sdk::horizon_client::HorizonClient;
/// # use stellar_rust_sdk::ledgers::prelude::*;
/// # use stellar_rust_sdk::models::Request;
/// #
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org".to_string())?;
/// #
/// # let all_ledgers_request = LedgersRequest::new().set_limit(10)?;
/// let ledgers_response = horizon_client.get_all_ledgers(&all_ledgers_request).await;
///
/// // Process the response
/// match ledgers_response {
///     Ok(response) => {
///         for ledger in response._embedded().records() {
///             println!("Ledger ID: {}", ledger.id());
///             // Further processing...
///         }
///     }
///     Err(e) => println!("Error parsing response: {}", e),
/// }
/// # Ok(())
/// # }
/// ```
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
/// # Usage
///
/// To utilize this module, create an instance of `SingleLedgerRequest`, set the ledger sequence 
/// number using the `set_sequence` method, and pass it to the `HorizonClient::get_single_ledger` 
/// method. This method will return a `SingleLedgerResponse`, which contains the detailed 
/// information of the specified ledger.
///
/// # Example
///
/// ```
/// # use stellar_rust_sdk::ledgers::single_ledger_request::SingleLedgerRequest;
/// # use stellar_rust_sdk::horizon_client::HorizonClient;
/// # use stellar_rust_sdk::models::Request;
/// # 
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// #     let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org".to_string())?;
/// let ledger_sequence = 1234567; // Replace with the desired ledger sequence number
/// let request = SingleLedgerRequest::new().set_sequence(ledger_sequence)?;
///
/// let ledger_response = horizon_client.get_single_ledger(&request).await?;
/// // Process the ledger response here
/// #     Ok(())
/// # }
/// ```
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
/// # Usage
///
/// This structure is primarily used internally by the `HorizonClient` to process responses from 
/// single ledger-related API calls. After making a request for a specific ledger using the 
/// `SingleLedgerRequest`, the response from the Horizon server is deserialized into an instance of 
/// `SingleLedgerResponse`.
///
/// # Example
///
/// Assuming you have already made a request using `SingleLedgerRequest` and have a valid `HorizonClient`:
/// ```
/// # use stellar_rust_sdk::ledgers::single_ledger_request::{SingleLedgerRequest, Sequence};
/// # use stellar_rust_sdk::ledgers::single_ledger_response::SingleLedgerResponse;
/// # use stellar_rust_sdk::horizon_client::HorizonClient;
/// # use stellar_rust_sdk::models::Request;
/// # 
/// # async fn fetch_ledger_details() -> Result<(), Box<dyn std::error::Error>> {
/// #     let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org".to_string())?;
/// # let request = SingleLedgerRequest::new().set_sequence(123456)?;
/// let ledger_response: Result<SingleLedgerResponse, String> = horizon_client.get_single_ledger(&request).await;
///
/// if let Ok(ledger) = ledger_response {
///     println!("Ledger ID: {}", ledger.id());
///     // Additional processing...
/// }
/// #     Ok(())
/// # }
/// ```
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
/// use stellar_rust_sdk::ledgers::prelude::*;
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
