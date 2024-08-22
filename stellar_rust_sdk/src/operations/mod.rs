/// Provides the `AllOperationsRequest`.
///
/// # Usage
/// This module provides the `AllOperationsRequest` struct, specifically designed for
/// constructing requests to query information about all operations from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_all_operations`](crate::horizon_client::HorizonClient::get_all_operations)
/// method.
///
pub mod all_operations_request;

/// Provides the `OperationsForAccountRequest`.
///
/// # Usage
/// This module provides the `OperationsForAccountRequest` struct, specifically designed for
/// constructing requests to query information about all operations for a given account from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_operations_for_account`](crate::horizon_client::HorizonClient::get_operations_for_account)
/// method.
///
pub mod operations_for_account_request;

/// Provides the `OperationsForLedgerRequest`.
///
/// # Usage
/// This module provides the `OperationsForLedgerRequest` struct, specifically designed for
/// constructing requests to query information about all operations for a given ledger from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_operations_for_ledger`](crate::horizon_client::HorizonClient::get_operations_for_ledger)
/// method.
///
pub mod operations_for_ledger_request;

/// Provides the `OperationsForLiquidityPoolRequest`.
///
/// # Usage
/// This module provides the `OperationsForLiquidityPoolRequest` struct, specifically designed for
/// constructing requests to query information about all operations for a given liquidity pool from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_operations_for_liquidity_pool`](crate::horizon_client::HorizonClient::get_operations_for_liquidity_pool)
/// method.
///
pub mod operations_for_liquidity_pool_request;

/// Provides the `OperationsForTransactionRequest`.
///
/// # Usage
/// This module provides the `OperationsForTransactionRequest` struct, specifically designed for
/// constructing requests to query information about all operations for a given transaction from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_operations_for_transaction`](crate::horizon_client::HorizonClient::get_operations_for_transaction)
/// method.
///
pub mod operations_for_transaction_request;

/// Provides the responses.
///
/// This module defines structures representing the response from the Horizon API when querying
/// for operations. The structures are designed to deserialize the JSON response into Rust
/// objects, enabling straightforward access to various details of a single transaction.
///
/// # Usage
/// These structures are equipped with serialization capabilities to handle the JSON data from the
/// Horizon server and with getter methods for easy field access.
pub mod response;

/// Provides the `SingleOperationRequest`.
///
/// # Usage
/// This module provides the `SingleOperationRequest` struct, specifically designed for
/// constructing requests to query information about a single operation from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_single_operation`](crate::horizon_client::HorizonClient::get_single_operation)
/// method.
///
pub mod single_operation_request;

/// The base path for operation-related endpoints in the Horizon API.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for transaction-related API calls.
static OPERATIONS_PATH: &str = "operations";

/// The `prelude` module of the `operations` module.
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
/// * From `single_operation_request`: All items (e.g. `SingleOperationRequest`).
/// * From `all_operations_request`: All items (e.g. `AllOperationsRequest`).
/// * From `operations_for_account_request`: All items (e.g. `OperationsForAccountRequest`).
/// * From `operations_for_ledger_request`: All items (e.g. `OperationsForLedgerRequest`).
/// * From `operations_for_liquidity_pool_request`: All items (e.g. `OperationsForLiquidityPoolRequest`).
/// * From `operations_for_transaction_request`: All items (e.g. `OperationsForTransactionRequest`).
/// * From `response`: All items (e.g. `OperationResponse`, `Operation`, etc.).
///
/// # Example
/// ```
/// # use crate::stellar_rs::models::*;
/// // Import the contents of the operations prelude
/// use stellar_rs::operations::prelude::*;
///
/// // Now you can directly use AllOperationsRequest, OperationResponse, etc.
/// let all_operations_request = AllOperationsRequest::new();
/// ```
pub mod prelude {
    pub use super::all_operations_request::*;
    pub use super::operations_for_account_request::*;
    pub use super::operations_for_ledger_request::*;
    pub use super::operations_for_liquidity_pool_request::*;
    pub use super::operations_for_transaction_request::*;
    pub use super::response::*;
    pub use super::single_operation_request::*;
}

#[cfg(test)]
pub mod tests {
    use crate::{
        horizon_client::*,
        operations::{
            operations_for_account_request::OperationsForAccountRequest,
            prelude::{
                AllOperationsRequest,
                OperationsForLedgerRequest,
                OperationsForLiquidityPoolRequest,
                OperationsForTransactionRequest,            },
            response::{Operation, OperationResponse},
            single_operation_request::SingleOperationRequest,
        },
    };

    #[tokio::test]
    async fn test_get_all_operations() {
        const ID: &str = "2314987376641";
        const PAGING_TOKEN: &str = "2314987376641";
        const TRANSACTION_SUCCESFULL: bool = true;
        const SOURCE_ACCOUNT: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const TYPE: &str = "create_account";
        const TYPE_I: i64 = 0;
        const CREATED_AT: &str = "2024-06-11T21:36:12Z";
        const TRANSACTION_HASH: &str =
            "b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const STARTING_BALANCE: &str = "10000000000.0000000";
        const FUNDER: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string())
                .unwrap();

        let all_operations_request = AllOperationsRequest::new().set_limit(2).unwrap();

        let all_operations_response: Result<OperationResponse, String> = horizon_client
            .get_all_operations(&all_operations_request)
            .await;

        assert!(all_operations_response.is_ok());

        let binding = all_operations_response.unwrap();
        let all_operations_response = &binding.embedded().records()[0];

        assert_eq!(all_operations_response.id(), ID);
        assert_eq!(all_operations_response.paging_token(), PAGING_TOKEN);
        assert_eq!(
            all_operations_response.transaction_successful(),
            &TRANSACTION_SUCCESFULL
        );
        assert_eq!(all_operations_response.source_account(), SOURCE_ACCOUNT);
        assert_eq!(all_operations_response.type_field(), TYPE);
        assert_eq!(all_operations_response.type_i(), &TYPE_I);
        assert_eq!(all_operations_response.created_at(), CREATED_AT);
        assert_eq!(all_operations_response.transaction_hash(), TRANSACTION_HASH);
        assert_eq!(all_operations_response.starting_balance(), STARTING_BALANCE);
        assert_eq!(all_operations_response.funder(), FUNDER);
        assert_eq!(all_operations_response.account(), ACCOUNT);
    }

    #[tokio::test]
    async fn test_get_single_operation() {
        const ID: &str = "2314987376641";
        const PAGING_TOKEN: &str = "2314987376641";
        const TRANSACTION_SUCCESFULL: bool = true;
        const SOURCE_ACCOUNT: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const TYPE: &str = "create_account";
        const TYPE_I: i64 = 0;
        const CREATED_AT: &str = "2024-06-11T21:36:12Z";
        const TRANSACTION_HASH: &str =
            "b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const STARTING_BALANCE: &str = "10000000000.0000000";
        const FUNDER: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string())
                .unwrap();

        let single_operation_request =
            SingleOperationRequest::new().set_operation_id(ID.to_string());

        let all_operations_response: Result<Operation, String> = horizon_client
            .get_single_operation(&single_operation_request)
            .await;

        assert!(all_operations_response.is_ok());

        let binding = all_operations_response.unwrap();

        assert_eq!(binding.id(), ID);
        assert_eq!(binding.paging_token(), PAGING_TOKEN);
        assert_eq!(binding.transaction_successful(), &TRANSACTION_SUCCESFULL);
        assert_eq!(binding.source_account(), SOURCE_ACCOUNT);
        assert_eq!(binding.type_field(), TYPE);
        assert_eq!(binding.type_i(), &TYPE_I);
        assert_eq!(binding.created_at(), CREATED_AT);
        assert_eq!(binding.transaction_hash(), TRANSACTION_HASH);
        assert_eq!(binding.starting_balance(), STARTING_BALANCE);
        assert_eq!(binding.funder(), FUNDER);
        assert_eq!(binding.account(), ACCOUNT);
    }

    #[tokio::test]
    async fn test_get_operations_for_account() {
        const ACCOUNT_ID: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const ID: &str = "2314987376641";
        const PAGING_TOKEN: &str = "2314987376641";
        const TRANSACTION_SUCCESFULL: bool = true;
        const SOURCE_ACCOUNT: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const TYPE: &str = "create_account";
        const TYPE_I: i64 = 0;
        const CREATED_AT: &str = "2024-06-11T21:36:12Z";
        const TRANSACTION_HASH: &str =
            "b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const STARTING_BALANCE: &str = "10000000000.0000000";
        const FUNDER: &str = ACCOUNT_ID;
        const ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string())
                .unwrap();

        let operations_for_account_request = OperationsForAccountRequest::new()
            .set_account_id(ACCOUNT_ID.to_string())
            .set_limit(2)
            .unwrap()
            .set_cursor(2)
            .unwrap();

        let operation_for_account_response = horizon_client
            .get_operations_for_account(&operations_for_account_request)
            .await;

        assert!(operation_for_account_response.is_ok());

        let binding = operation_for_account_response.unwrap();
        let operation_for_account_response = &binding.embedded().records()[0];

        assert_eq!(operation_for_account_response.id(), ID);
        assert_eq!(operation_for_account_response.paging_token(), PAGING_TOKEN);
        assert_eq!(
            operation_for_account_response.transaction_successful(),
            &TRANSACTION_SUCCESFULL
        );
        assert_eq!(
            operation_for_account_response.source_account(),
            SOURCE_ACCOUNT
        );
        assert_eq!(operation_for_account_response.type_field(), TYPE);
        assert_eq!(operation_for_account_response.type_i(), &TYPE_I);
        assert_eq!(operation_for_account_response.created_at(), CREATED_AT);
        assert_eq!(
            operation_for_account_response.transaction_hash(),
            TRANSACTION_HASH
        );
        assert_eq!(
            operation_for_account_response.starting_balance(),
            STARTING_BALANCE
        );
        assert_eq!(operation_for_account_response.funder(), FUNDER);
        assert_eq!(operation_for_account_response.account(), ACCOUNT);
    }

    #[tokio::test]
    async fn test_get_operations_for_ledger() {
        const ID: &str = "2314987376641";
        const PAGING_TOKEN: &str = "2314987376641";
        const TRANSACTION_SUCCESFULL: bool = true;
        const SOURCE_ACCOUNT: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const TYPE: &str = "create_account";
        const TYPE_I: i64 = 0;
        const CREATED_AT: &str = "2024-06-11T21:36:12Z";
        const TRANSACTION_HASH: &str =
            "b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const STARTING_BALANCE: &str = "10000000000.0000000";
        const FUNDER: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string())
                .unwrap();

        let operations_for_ledger_request = OperationsForLedgerRequest::new().set_limit(2).unwrap();

        let operation_for_ledger_response = horizon_client
            .get_operations_for_ledger(&operations_for_ledger_request)
            .await;

        assert!(operation_for_ledger_response.is_ok());

        let binding = operation_for_ledger_response.unwrap();
        let operation_for_ledger_response = &binding.embedded().records()[0];

        assert_eq!(operation_for_ledger_response.id(), ID);
        assert_eq!(operation_for_ledger_response.paging_token(), PAGING_TOKEN);
        assert_eq!(
            operation_for_ledger_response.transaction_successful(),
            &TRANSACTION_SUCCESFULL
        );
        assert_eq!(
            operation_for_ledger_response.source_account(),
            SOURCE_ACCOUNT
        );
        assert_eq!(operation_for_ledger_response.type_field(), TYPE);
        assert_eq!(operation_for_ledger_response.type_i(), &TYPE_I);
        assert_eq!(operation_for_ledger_response.created_at(), CREATED_AT);
        assert_eq!(
            operation_for_ledger_response.transaction_hash(),
            TRANSACTION_HASH
        );
        assert_eq!(
            operation_for_ledger_response.starting_balance(),
            STARTING_BALANCE
        );
        assert_eq!(operation_for_ledger_response.funder(), FUNDER);
        assert_eq!(operation_for_ledger_response.account(), ACCOUNT);
    }

    #[tokio::test]
    async fn test_get_operations_for_liquidity_pool() {
        const ID: &str = "2314987376641";
        const PAGING_TOKEN: &str = "2314987376641";
        const TRANSACTION_SUCCESFULL: bool = true;
        const SOURCE_ACCOUNT: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const TYPE: &str = "create_account";
        const TYPE_I: i64 = 0;
        const CREATED_AT: &str = "2024-06-11T21:36:12Z";
        const TRANSACTION_HASH: &str =
            "b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const STARTING_BALANCE: &str = "10000000000.0000000";
        const FUNDER: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string())
                .unwrap();

        let operations_for_liquidity_pool_request = OperationsForLiquidityPoolRequest::new()
            .set_limit(2)
            .unwrap();

        let operation_for_liquidity_pool_response = horizon_client
            .get_operations_for_liquidity_pool(&operations_for_liquidity_pool_request)
            .await;

        assert!(operation_for_liquidity_pool_response.is_ok());

        let binding = operation_for_liquidity_pool_response.unwrap();
        let operation_for_liquidity_pool_response = &binding.embedded().records()[0];

        assert_eq!(operation_for_liquidity_pool_response.id(), ID);
        assert_eq!(
            operation_for_liquidity_pool_response.paging_token(),
            PAGING_TOKEN
        );
        assert_eq!(
            operation_for_liquidity_pool_response.transaction_successful(),
            &TRANSACTION_SUCCESFULL
        );
        assert_eq!(
            operation_for_liquidity_pool_response.source_account(),
            SOURCE_ACCOUNT
        );
        assert_eq!(operation_for_liquidity_pool_response.type_field(), TYPE);
        assert_eq!(operation_for_liquidity_pool_response.type_i(), &TYPE_I);
        assert_eq!(
            operation_for_liquidity_pool_response.created_at(),
            CREATED_AT
        );
        assert_eq!(
            operation_for_liquidity_pool_response.transaction_hash(),
            TRANSACTION_HASH
        );
        assert_eq!(
            operation_for_liquidity_pool_response.starting_balance(),
            STARTING_BALANCE
        );
        assert_eq!(operation_for_liquidity_pool_response.funder(), FUNDER);
        assert_eq!(operation_for_liquidity_pool_response.account(), ACCOUNT);
    }

    #[tokio::test]
    async fn test_get_operations_for_transaction() {
        const REQUEST_TRANSACTION_HASH: &str = "b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const ID: &str = "2314987376641";
        const PAGING_TOKEN: &str = "2314987376641";
        const TRANSACTION_SUCCESFULL: bool = true;
        const SOURCE_ACCOUNT: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const TYPE: &str = "create_account";
        const TYPE_I: i64 = 0;
        const CREATED_AT: &str = "2024-06-11T21:36:12Z";
        const TRANSACTION_HASH: &str =
            "b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const STARTING_BALANCE: &str = "10000000000.0000000";
        const FUNDER: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
        const ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";

        let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org".to_string())
                .unwrap();

        let operations_for_transaction_request = OperationsForTransactionRequest::new()
            .set_transaction_hash(REQUEST_TRANSACTION_HASH.to_string())
            .set_limit(2)
            .unwrap();

        let operation_for_liquidity_pool_response = horizon_client
            .get_operations_for_transaction(&operations_for_transaction_request)
            .await;

        assert!(operation_for_liquidity_pool_response.is_ok());

        let binding = operation_for_liquidity_pool_response.unwrap();
        let operation_for_liquidity_pool_response = &binding.embedded().records()[0];

        assert_eq!(operation_for_liquidity_pool_response.id(), ID);
        assert_eq!(
            operation_for_liquidity_pool_response.paging_token(),
            PAGING_TOKEN
        );
        assert_eq!(
            operation_for_liquidity_pool_response.transaction_successful(),
            &TRANSACTION_SUCCESFULL
        );
        assert_eq!(
            operation_for_liquidity_pool_response.source_account(),
            SOURCE_ACCOUNT
        );
        assert_eq!(operation_for_liquidity_pool_response.type_field(), TYPE);
        assert_eq!(operation_for_liquidity_pool_response.type_i(), &TYPE_I);
        assert_eq!(
            operation_for_liquidity_pool_response.created_at(),
            CREATED_AT
        );
        assert_eq!(
            operation_for_liquidity_pool_response.transaction_hash(),
            TRANSACTION_HASH
        );
        assert_eq!(
            operation_for_liquidity_pool_response.starting_balance(),
            STARTING_BALANCE
        );
        assert_eq!(operation_for_liquidity_pool_response.funder(), FUNDER);
        assert_eq!(operation_for_liquidity_pool_response.account(), ACCOUNT);
    }
}
