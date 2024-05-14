/// Provides the `SingleTransactionRequest`.
///
/// # Usage
/// This module provides the `SingleTransactionRequest` struct, specifically designed for
/// constructing requests to query information about a single transaction from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_single_transaction`](crate::horizon_client::HorizonClient::get_single_transaction)
/// method.
///
pub mod single_transaction_request;

/// Provides the responses.
///
/// This module defines structures representing the response from the Horizon API when querying
/// for transactions. The structures are designed to deserialize the JSON response into Rust
/// objects, enabling straightforward access to various details of a single transaction.
///
/// # Usage
/// These structures are equipped with serialization capabilities to handle the JSON data from the
/// Horizon server and with getter methods for easy field access.
pub mod response;

/// The base path for transaction-related endpoints in the Horizon API.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for offer-related API calls.
static TRANSACTIONS_PATH: &str = "transactions";

/// The `prelude` module of the `transactions` module.
///
/// # Usage
/// This module serves as a convenience for users of the Horizon Rust SDK, allowing for easy and
/// ergonomic import of the most commonly used items across various modules. It re-exports
/// key structs and traits from the sibling modules, simplifying access to these components
/// when using the library.
///
/// By importing the contents of `prelude`, users can conveniently access the primary
/// functionalities of the transaction-related modules without needing to import each item
/// individually.
///
/// # Contents
///
/// The `prelude` includes the following re-exports:
///
/// * From `single_transaction_request`: All items (e.g. `SingleTransactionRequest`).
/// * From `response`: All items (e.g. `SingleTransactionResponse`, `Preconditions`, etc.).
///
/// # Example
/// ```
/// # use crate::stellar_rs::models::*;
/// // Import the contents of the transactions prelude
/// use stellar_rs::transactions::prelude::*;
///
/// // Now you can directly use SingleTransactionRequest, SingleTransactionResponse, etc.
/// let single_transactions_request = SingleTransactionRequest::new();
/// ```
pub mod prelude {
    pub use super::single_transaction_request::*;
    pub use super::response::*;
}

#[cfg(test)]
pub mod test {
    use super::prelude::*;
    use crate::horizon_client::HorizonClient;

    #[tokio::test]
    async fn test_get_single_transaction() {
        const LINK_SELF: &str = "https://horizon-testnet.stellar.org/transactions/be0d59c8706e8fd525d2ab10910a55ec57323663858c65b330a3f93afb13ab0f";
        const LINK_ACCOUNT: &str = "https://horizon-testnet.stellar.org/accounts/GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";
        const LINK_LEDGER: &str = "https://horizon-testnet.stellar.org/ledgers/126";
        const LINK_OPERATIONS: &str = "https://horizon-testnet.stellar.org/transactions/be0d59c8706e8fd525d2ab10910a55ec57323663858c65b330a3f93afb13ab0f/operations{?cursor,limit,order}";
        const LINK_EFFECTS: &str = "https://horizon-testnet.stellar.org/transactions/be0d59c8706e8fd525d2ab10910a55ec57323663858c65b330a3f93afb13ab0f/effects{?cursor,limit,order}";
        const LINK_PRECEDES: &str = "https://horizon-testnet.stellar.org/transactions?order=asc&cursor=541165883392";
        const LINK_SUCCEEDS: &str = "https://horizon-testnet.stellar.org/transactions?order=desc&cursor=541165883392";
        const LINK_TRANSACTION: &str = "https://horizon-testnet.stellar.org/transactions/be0d59c8706e8fd525d2ab10910a55ec57323663858c65b330a3f93afb13ab0f";
        const ID: &str = "be0d59c8706e8fd525d2ab10910a55ec57323663858c65b330a3f93afb13ab0f";
        const PAGING_TOKEN: &str = "541165883392";
        const SUCCESSFUL: &bool = &true;
        const HASH: &str = "be0d59c8706e8fd525d2ab10910a55ec57323663858c65b330a3f93afb13ab0f";
        const LEDGER: &i64 = &126;
        const CREATED_AT: &str = "2024-02-06T17:44:28Z";
        const SOURCE_ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";
        const SOURCE_ACCOUNT_SEQUENCE: &str = "459561500680";
        const FEE_ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";
        const FEE_CHARGED: &str = "2000";
        const MAX_FEE: &str = "2000";
        const OPERATION_COUNT: &i64 = &20;
        // TODO: Is it necessary to test the following 4 values, as they're very long?
        // const ENVELOPE_XDR: &str = "";
        // const RESULT_XDR: &str = "";
        // const RESULT_META_XDR: &str = "";
        // const FEE_META_XDR: &str = "";
        const MEMO_TYPE: &str = "none";
        const SIGNATURE: &str = "wd3ANI0TfNZtfzPt7sBbeHm7tOmLtTCadyD0Roor3f6G/FqCO5poG+cuAbsxLm7cf9XBhobkC0Zdj4RgQMGbCQ==";
        const VALID_AFTER: &str = "1970-01-01T00:00:00Z";
        const VALID_BEFORE: &str = "2024-02-06T17:49:24Z";
        const MIN_TIME: &str = "0";
        const MAX_TIME: &str = "1707241764";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org"
            .to_string())
            .unwrap();

        let single_transaction_request =
            SingleTransactionRequest::new()
            .set_transaction_hash(ID.to_string())
            .unwrap();

        let single_transaction_response = horizon_client
            .get_single_transaction(&single_transaction_request)
            .await;

        assert!(single_transaction_response.clone().is_ok());
        let response = single_transaction_response.unwrap();
        assert_eq!(response.links().self_link().href().as_ref().unwrap(), LINK_SELF);
        assert_eq!(response.links().account().href().as_ref().unwrap(), LINK_ACCOUNT);
        assert_eq!(response.links().ledger().href().as_ref().unwrap(), LINK_LEDGER);
        assert_eq!(response.links().operations().href().as_ref().unwrap(), LINK_OPERATIONS);
        assert_eq!(response.links().effects().href().as_ref().unwrap(), LINK_EFFECTS);
        assert_eq!(response.links().precedes().href().as_ref().unwrap(), LINK_PRECEDES);
        assert_eq!(response.links().succeeds().href().as_ref().unwrap(), LINK_SUCCEEDS);
        assert_eq!(response.links().transaction().href().as_ref().unwrap(), LINK_TRANSACTION);
        assert_eq!(response.id(), ID);
        assert_eq!(response.paging_token(), PAGING_TOKEN);
        assert_eq!(response.successful(), SUCCESSFUL);
        assert_eq!(response.hash(), HASH);
        assert_eq!(response.ledger(), LEDGER);
        assert_eq!(response.created_at(), CREATED_AT);
        assert_eq!(response.source_account(), SOURCE_ACCOUNT);
        assert_eq!(response.source_account_sequence(), SOURCE_ACCOUNT_SEQUENCE);
        assert_eq!(response.fee_account(), FEE_ACCOUNT);
        assert_eq!(response.fee_charged(), FEE_CHARGED);
        assert_eq!(response.max_fee(), MAX_FEE);
        assert_eq!(response.operation_count(), OPERATION_COUNT);
        assert_eq!(response.memo_type(), MEMO_TYPE);
        assert_eq!(response.signatures()[0], SIGNATURE);
        assert_eq!(response.valid_after(), VALID_AFTER);
        assert_eq!(response.valid_before(), VALID_BEFORE);
        assert_eq!(response.preconditions().timebounds().min_time(), MIN_TIME);
        assert_eq!(response.preconditions().timebounds().max_time().as_ref().unwrap(), MAX_TIME);
    }
}