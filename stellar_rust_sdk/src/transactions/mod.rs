/// Provides the `SingleTransactionRequest`.
///
/// # Usage
/// This module provides the `SingleTransactionRequest` struct, specifically designed for
/// constructing requests to query information about a single transaction from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_single_transaction`](crate::horizon_client::HorizonClient::get_single_transaction)
/// method.
///
pub mod single_transaction_request;

/// Provides the `AllTransactionsRequest`.
///
/// # Usage
/// This module provides the `AllTransactionsRequest` struct, specifically designed for
/// constructing requests to query information about all transactions from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_all_transactions`](crate::horizon_client::HorizonClient::get_all_transactions)
/// method.
///
pub mod all_transactions_request;

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
/// to ensure consistent and accurate path construction for transaction-related API calls.
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
/// * From `all_transactions_request`: All items (e.g. `AllTransactionsRequest`).
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
    pub use super::all_transactions_request::*;
    pub use super::response::*;
}

#[cfg(test)]
pub mod test {
    use super::prelude::*;
    use crate::horizon_client::HorizonClient;

    #[tokio::test]
    async fn test_get_single_transaction() {
        const LINK_SELF: &str = "https://horizon-testnet.stellar.org/transactions/b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const LINK_ACCOUNT: &str = "https://horizon-testnet.stellar.org/accounts/GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const LINK_LEDGER: &str = "https://horizon-testnet.stellar.org/ledgers/539";
        const LINK_OPERATIONS: &str = "https://horizon-testnet.stellar.org/transactions/b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020/operations{?cursor,limit,order}";
        const LINK_EFFECTS: &str = "https://horizon-testnet.stellar.org/transactions/b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020/effects{?cursor,limit,order}";
        const LINK_PRECEDES: &str = "https://horizon-testnet.stellar.org/transactions?order=asc&cursor=2314987376640";
        const LINK_SUCCEEDS: &str = "https://horizon-testnet.stellar.org/transactions?order=desc&cursor=2314987376640";
        const LINK_TRANSACTION: &str = "https://horizon-testnet.stellar.org/transactions/b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const ID: &str = "b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const PAGING_TOKEN: &str = "2314987376640";
        const SUCCESSFUL: &bool = &true;
        const HASH: &str = "b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const LEDGER: &i64 = &539;
        const CREATED_AT: &str = "2024-06-11T21:36:12Z";
        const SOURCE_ACCOUNT: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const SOURCE_ACCOUNT_SEQUENCE: &str = "1";
        const FEE_ACCOUNT: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const FEE_CHARGED: &str = "1100";
        const MAX_FEE: &str = "1100";
        const OPERATION_COUNT: &i64 = &11;
        // TODO: Is it necessary to test the following 4 values, as they're very long?
        // const ENVELOPE_XDR: &str = "";
        // const RESULT_XDR: &str = "";
        // const RESULT_META_XDR: &str = "";
        // const FEE_META_XDR: &str = "";
        const MEMO_TYPE: &str = "none";
        const SIGNATURE: &str = "NUHx9PZlcXQ9mq1lf1usrSTP4/gbxUqzUOQOSU/pQuy9dF7FcUF0fjEbzFECxHUcl4QEfbvyGIE029TA3DrODA==";
        const VALID_AFTER: &str = "1970-01-01T00:00:00Z";
        const MIN_TIME: &str = "0";

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
        assert_eq!(response.valid_after().as_ref().unwrap(), VALID_AFTER);
        assert_eq!(response.preconditions().as_ref().unwrap().timebounds().min_time(), MIN_TIME);
    }

    #[tokio::test]
    async fn test_get_all_transactions() {
        const LINK_SELF: &str = "https://horizon-testnet.stellar.org/transactions/b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const LINK_ACCOUNT: &str = "https://horizon-testnet.stellar.org/accounts/GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const LINK_LEDGER: &str = "https://horizon-testnet.stellar.org/ledgers/539";
        const LINK_OPERATIONS: &str = "https://horizon-testnet.stellar.org/transactions/b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020/operations{?cursor,limit,order}";
        const LINK_EFFECTS: &str =  "https://horizon-testnet.stellar.org/transactions/b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020/effects{?cursor,limit,order}";
        const LINK_PRECEDES: &str = "https://horizon-testnet.stellar.org/transactions?order=asc&cursor=2314987376640";
        const LINK_SUCCEEDS: &str = "https://horizon-testnet.stellar.org/transactions?order=desc&cursor=2314987376640";
        const LINK_TRANSACTION: &str = "https://horizon-testnet.stellar.org/transactions/b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const ID: &str = "b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const PAGING_TOKEN: &str = "2314987376640";
        const SUCCESSFUL: &bool = &true;
        const HASH: &str = "b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const LEDGER: &i64 = &539;
        const CREATED_AT: &str = "2024-06-11T21:36:12Z";
        const SOURCE_ACCOUNT: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const SOURCE_ACCOUNT_SEQUENCE: &str = "1";
        const FEE_ACCOUNT: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const FEE_CHARGED: &str = "1100";
        const MAX_FEE: &str = "1100";
        const OPERATION_COUNT: &i64 = &11;
        const MEMO_TYPE: &str = "none";
        const SIGNATURE: &str = "NUHx9PZlcXQ9mq1lf1usrSTP4/gbxUqzUOQOSU/pQuy9dF7FcUF0fjEbzFECxHUcl4QEfbvyGIE029TA3DrODA==";
        const VALID_AFTER: &str = "1970-01-01T00:00:00Z";
        const MIN_TIME: &str = "0";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org"
            .to_string())
            .unwrap();

        let all_transactions_request = AllTransactionsRequest::new()
            .set_include_failed(true).unwrap();

        let all_transactions_response = horizon_client
            .get_all_transactions(&all_transactions_request)
            .await;

        assert!(all_transactions_response.clone().is_ok());
        let binding = all_transactions_response.unwrap();
        let record = &binding.embedded().records()[0];
        assert_eq!(record.links().self_link().href().as_ref().unwrap(), LINK_SELF);
        assert_eq!(record.links().account().href().as_ref().unwrap(), LINK_ACCOUNT);
        assert_eq!(record.links().ledger().href().as_ref().unwrap(), LINK_LEDGER);
        assert_eq!(record.links().operations().href().as_ref().unwrap(), LINK_OPERATIONS);
        assert_eq!(record.links().effects().href().as_ref().unwrap(), LINK_EFFECTS);
        assert_eq!(record.links().precedes().href().as_ref().unwrap(), LINK_PRECEDES);
        assert_eq!(record.links().succeeds().href().as_ref().unwrap(), LINK_SUCCEEDS);
        assert_eq!(record.links().transaction().href().as_ref().unwrap(), LINK_TRANSACTION);
        assert_eq!(record.id(), ID);
        assert_eq!(record.paging_token(), PAGING_TOKEN);
        assert_eq!(record.successful(), SUCCESSFUL);
        assert_eq!(record.hash(), HASH);
        assert_eq!(record.ledger(), LEDGER);
        assert_eq!(record.created_at(), CREATED_AT);
        assert_eq!(record.source_account(), SOURCE_ACCOUNT);
        assert_eq!(record.source_account_sequence(), SOURCE_ACCOUNT_SEQUENCE);
        assert_eq!(record.fee_account(), FEE_ACCOUNT);
        assert_eq!(record.fee_charged(), FEE_CHARGED);
        assert_eq!(record.max_fee(), MAX_FEE);
        assert_eq!(record.operation_count(), OPERATION_COUNT);
        assert_eq!(record.memo_type(), MEMO_TYPE);
        assert_eq!(record.signatures()[0], SIGNATURE); // Check only the first signature of the vector
        assert_eq!(record.valid_after().as_ref().unwrap(), VALID_AFTER);
        assert_eq!(record.preconditions().as_ref().unwrap().timebounds().min_time(), MIN_TIME);
    }
}