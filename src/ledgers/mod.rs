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

/// Provides Ledger response models.
///
/// This module contains structures that represent the response received from the Horizon API when
/// querying for ledger data. These structures are designed to parse and encapsulate the JSON response
/// from the Horizon server, making it easier to work with ledger data in Rust applications.
///
/// The primary structure in this module is `LedgersResponse`, which contains detailed information about
/// each ledger, including its transactions, operations, and other related data.
///
pub mod response;

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
    pub use super::response::*;
    pub use super::single_ledger_request::*;
}

#[cfg(test)]
pub mod tests {
    use super::prelude::*;
    use crate::horizon_client::HorizonClient;
    use base64::{engine::general_purpose, Engine};

    #[tokio::test]
    async fn test_get_all_ledgers() {
        let hash = "f96c4021adc1ae496c662f4f97143e499a9548f541c64bb2401a1b1701de5150";
        let prev_hash = "63d98f536ee68d1b27b5b89f23af5311b7569a24faf1403ad0b52b633b07be99";

        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let all_ledgers_request = LedgersRequest::new().set_limit(2).unwrap();

        let all_ledgers_response = horizon_client.get_all_ledgers(&all_ledgers_request).await;
        assert!(all_ledgers_response.is_ok());

        let binding = all_ledgers_response.unwrap();
        let all_ledgers_response = &binding.embedded().records()[0];

        assert_eq!(all_ledgers_response.hash(), hash);
        assert_eq!(all_ledgers_response.prev_hash(), prev_hash);
        assert_eq!(all_ledgers_response.sequence(), &2);
        assert_eq!(all_ledgers_response.successful_transaction_count(), &0);
        assert_eq!(all_ledgers_response.paging_token(), "8589934592");
    }

    #[tokio::test]
    async fn test_get_single_ledger() {
        let id = "f96c4021adc1ae496c662f4f97143e499a9548f541c64bb2401a1b1701de5150";
        let hash = "f96c4021adc1ae496c662f4f97143e499a9548f541c64bb2401a1b1701de5150";
        let prev_hash = "63d98f536ee68d1b27b5b89f23af5311b7569a24faf1403ad0b52b633b07be99";
        let closed_at = "2024-02-06T17:32:26Z";
        let closed_at_timepoint = 1707240746;

        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let single_ledger_request = SingleLedgerRequest::new().set_sequence(2).unwrap();

        let single_ledger_response = horizon_client
            .get_single_ledger(&single_ledger_request)
            .await;

        assert!(single_ledger_response.is_ok());
        let single_ledger_response = single_ledger_response.unwrap();
        assert_eq!(single_ledger_response.id(), id);
        assert_eq!(single_ledger_response.paging_token(), "8589934592");
        assert_eq!(single_ledger_response.hash(), hash);
        assert_eq!(single_ledger_response.prev_hash(), prev_hash);
        assert_eq!(single_ledger_response.sequence(), &2);
        assert_eq!(single_ledger_response.successful_transaction_count(), &0);
        assert_eq!(single_ledger_response.failed_transaction_count(), &0);
        assert_eq!(single_ledger_response.operation_count(), &0);
        assert_eq!(single_ledger_response.tx_set_operation_count(), &0);
        assert_eq!(single_ledger_response.closed_at(), closed_at);
        assert_eq!(single_ledger_response.total_coins(), "100000000000.0000000");
        assert_eq!(single_ledger_response.fee_pool(), "0.0000000");
        assert_eq!(single_ledger_response.base_fee_in_stroops(), &100);
        assert_eq!(single_ledger_response.base_reserve_in_stroops(), &100000000);
        assert_eq!(single_ledger_response.max_tx_set_size(), &100);
        assert_eq!(single_ledger_response.protocol_version(), &0);

        let decoded_xdr_header = single_ledger_response.decoded_header_xdr().unwrap();

        assert_eq!(
            decoded_xdr_header.bucket_list_hash.to_string(),
            "735227ed398461291237687b08446aa2c9b096e0c98a462dadda569f05dd2484"
        );

        assert_eq!(decoded_xdr_header.ledger_seq, 2);
        assert_eq!(decoded_xdr_header.total_coins, 1000000000000000000);
        assert_eq!(decoded_xdr_header.fee_pool, 0);
        assert_eq!(decoded_xdr_header.inflation_seq, 0);
        assert_eq!(decoded_xdr_header.id_pool, 0);
        assert_eq!(decoded_xdr_header.base_fee, 100);
        assert_eq!(decoded_xdr_header.base_reserve, 100000000);
        assert_eq!(decoded_xdr_header.max_tx_set_size, 100);

        let tx_set_hash = decoded_xdr_header.scp_value.tx_set_hash.to_string();
        let tx_set_hash_bytes = hex::decode(tx_set_hash.clone()).expect("Failed to decode hex");
        let tx_set_hash_base64 = general_purpose::STANDARD.encode(tx_set_hash_bytes.clone());

        assert_eq!(
            tx_set_hash_base64,
            "uZRHr9UdXKbTKiclfOjy72YZFJUkJPVcKT5htvorm1Q="
        );

        assert_eq!(
            decoded_xdr_header.scp_value.close_time,
            stellar_xdr::curr::TimePoint(closed_at_timepoint)
        );

        assert_eq!(
            decoded_xdr_header.ext,
            stellar_xdr::curr::LedgerHeaderExt::V0
        );
        for decoded in decoded_xdr_header.skip_list {
            assert_eq!(
                decoded.to_string(),
                "0000000000000000000000000000000000000000000000000000000000000000"
            );
        }
    }
}

