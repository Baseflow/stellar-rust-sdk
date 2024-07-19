/// Provides the `AllEffectsRequest` struct.
///
/// This module contains the `AllEffectsRequest` struct, which is designed to create requests
/// for querying comprehensive lists of effects from the Horizon server. 
///
/// The `AllEffectsRequest` struct is meant to be used in conjunction with the [`HorizonClient`](crate::horizon_client::HorizonClient)
/// to perform the actual API calls and fetch asset data. It adheres to the structure
/// and requirements of the Horizon API for asset queries.
///
pub mod all_effects_request;

/// Provides the `EffectsForAccountRequest`.
///
/// This module provides the `EffectsForAccountRequest` struct, specifically designed for
/// constructing requests to query information about effects of a specific account from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_effects_for_account`](crate::horizon_client::HorizonClient::get_effects_for_account)
/// method.
///
pub mod effects_for_account_request;

/// Provides the `EffectsForLedgerRequest`.
///
/// This module provides the `EffectsForLedgerRequest` struct, specifically designed for
/// constructing requests to query information about effects of a ledger from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_effects_for_ledger`](crate::horizon_client::HorizonClient::get_effects_for_ledger)
/// method.
///
pub mod effects_for_ledger_request;

/// Provides the `EffectsForLiquidityPoolRequest`.
///
/// This module provides the `EffectsForLiquidityPoolRequest` struct, specifically designed for
/// constructing requests to query information about effects referencing a given liquidity pool from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_effects_for_liquidity_pools`](crate::horizon_client::HorizonClient::get_effects_for_liquidity_pools)
/// method.
///
pub mod effects_for_liquidity_pools_request;

/// Provides the `EffectsForOperationRequest`.
///
/// This module provides the `EffectsForOperationRequest` struct, specifically designed for
/// constructing requests to query information about effects of a specific operation from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_effects_for_operation`](crate::horizon_client::HorizonClient::get_effects_for_operation)
/// method.
///
pub mod effects_for_operation_request;

/// Provides the `EffectsForTransactionRequest`.
///
/// This module provides the `EffectsForTransactionRequest` struct, specifically designed for
/// constructing requests to query information about effects of a specific transaction from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_effects_for_transaction`](crate::horizon_client::HorizonClient::get_effects_for_transaction)
/// method.
///
pub mod effects_for_transaction_request;

/// Provides the responses.
///
/// This module defines structures representing the response from the Horizon API when querying
/// for effects. The structures are designed to deserialize the JSON response into Rust
/// objects, enabling straightforward access to various details of a single Stellar account.
///
/// These structures are equipped with serialization capabilities to handle the JSON data from the
/// Horizon server and with getter methods for easy field access.
///
pub mod response;

/// The base path for effect-related endpoints in the Horizon API.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for effect-related API calls.
///
static EFFECTS_PATH: &str = "effects";

/// The `prelude` module of the `effects` module.
///
/// This module serves as a convenience for users of the Horizon Rust SDK, allowing for easy and
/// ergonomic import of the most commonly used items across various modules. It re-exports
/// key structs and traits from the sibling modules, simplifying access to these components
/// when using the library.
///
/// By importing the contents of `prelude`, users can conveniently access the primary
/// functionalities of the effect-related modules without needing to import each item
/// individually.
///
/// # Contents
///
/// The `prelude` includes the following re-exports:
///
/// * From `all_effects_request`: All items (e.g. `AllEffectsRequest`).
/// * From `effects_for_account_request`: All items (e.g. `EffectsForAccountRequest`).
/// * From `effects_for_ledger_request`: All items (e.g. `EffectsForLedgerRequest`).
/// * From `effects_for_liquidity_pools_request`: All items (e.g. `EffectsForLiquidityPoolRequest`).
/// * From `effects_for_operation_request`: All items (e.g. `EffectsForOperationRequest`).
/// * From `effects_for_transaction_request`: All items (e.g. `EffectForTransactionRequest`).
/// * From `response`: All items (e.g. `EffectsResponse`, `Effect`, `EffectLink`).
///
/// # Example
/// ```
/// # use crate::stellar_rs::models::*;
/// // Import the contents of the effects prelude.
/// use stellar_rs::effects::prelude::*;
///
/// // Now you can directly use AllEffectsRequest, EffectsForAccountRequest, etc.
/// let all_effects_request = AllEffectsRequest::new();
/// ```
///
pub mod prelude {
    pub use super::all_effects_request::*;
    pub use super::effects_for_account_request::*;
    pub use super::effects_for_ledger_request::*;
    pub use super::effects_for_liquidity_pools_request::*;
    pub use super::effects_for_operation_request::*;
    pub use super::effects_for_transaction_request::*;
    pub use super::response::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use crate::{horizon_client::HorizonClient, Paginatable};

    #[test]
    fn dummy_test() {
        assert_eq!(super::EFFECTS_PATH, "effects");
    }

    #[tokio::test]
    async fn test_get_all_effects() {
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let num_records_to_fetch = 2;

        let all_effects_request = AllEffectsRequest::new()
            .set_limit(num_records_to_fetch)
            .unwrap();
        let _all_effects_response = horizon_client.get_all_effects(&all_effects_request).await;

        assert!(_all_effects_response.clone().is_ok());

        // make sure there are actually 2 records
        assert_eq!(
            _all_effects_response
                .clone()
                .unwrap()
                .embedded()
                .records()
                .len() as u8,
            num_records_to_fetch
        );

        // test first record retrieved
        assert_eq!(
            _all_effects_response.clone().unwrap().embedded().records()[0].type_i,
            0
        );

        // test second record retrieved
        assert_eq!(
            _all_effects_response.clone().unwrap().embedded().records()[1].type_i,
            3
        );
    }

    #[tokio::test]
    async fn test_get_effects_for_account() {
        const ID: &str = "0000002314987376641-0000000001";
        const PAGING_TOKEN: &str = "2314987376641-1";
        const ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";
        const RECORD_TYPE: &str = "account_created";
        const TYPE_I: u32 = 0;
        const CREATED_AT: &str = "2024-06-11T21:36:12Z";
        const STARTING_BALANCE: &str = "10000000000.0000000";
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let effects_for_account_request = EffectsForAccountRequest::new().set_limit(2).unwrap();

        let effects_for_account_response = horizon_client
            .get_effects_for_account(&effects_for_account_request)
            .await;

        assert!(&effects_for_account_response.is_ok());
        let binding = effects_for_account_response.clone().unwrap();
        let record = &binding.embedded().records()[0];

        assert_eq!(record.id(), ID);
        assert_eq!(record.paging_token(), PAGING_TOKEN);
        assert_eq!(record.account(), ACCOUNT);
        assert_eq!(record.effect_type(), RECORD_TYPE);
        assert_eq!(record.type_i(), &TYPE_I);
        assert_eq!(record.created_at(), CREATED_AT);
        assert_eq!(
            record.starting_balance().as_ref().unwrap(),
            &STARTING_BALANCE
        );
    }

    #[tokio::test]
    async fn get_effects_for_liquidity_pools() {
        const ID: &str = "0000002314987376641-0000000001";
        const PAGING_TOKEN: &str = "2314987376641-1";
        const ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";
        const RECORD_TYPE: &str = "account_created";
        const TYPE_I: u32 = 0;
        const CREATED_AT: &str = "2024-06-11T21:36:12Z";
        const STARTING_BALANCE: &str = "10000000000.0000000";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let effects_for_liquidity_pools_request =
            EffectsForLiquidityPoolRequest::new().set_limit(2).unwrap();
        let effects_for_liquidity_pools_response = horizon_client
            .get_effects_for_liquidity_pools(&effects_for_liquidity_pools_request)
            .await;

        assert!(effects_for_liquidity_pools_response.is_ok());
        let binding = effects_for_liquidity_pools_response.clone().unwrap();
        let record = &binding.embedded().records()[0];

        assert_eq!(record.id(), ID);
        assert_eq!(record.paging_token(), PAGING_TOKEN);
        assert_eq!(record.account(), ACCOUNT);
        assert_eq!(record.effect_type(), RECORD_TYPE);
        assert_eq!(record.type_i(), &TYPE_I);
        assert_eq!(record.created_at(), CREATED_AT);
        assert_eq!(
            record.starting_balance().as_ref().unwrap(),
            &STARTING_BALANCE
        );

        // TODO: LEONARD FIX
        let _effects_for_liquidity_pools_request_with_id = EffectsForLiquidityPoolRequest::new()
            .set_limit(2)
            .expect("REASON")
            .set_liquidity_pool_id("0000000459561504769-0000000001".to_string());
        let effects_for_liquidity_pools_response = horizon_client
            .get_effects_for_liquidity_pools(&effects_for_liquidity_pools_request)
            .await;

        assert!(effects_for_liquidity_pools_response.clone().is_ok());
        assert_eq!(record.id(), ID);
    }

    #[tokio::test]
    async fn test_get_effects_for_ledger() {
        // found by trial and error in the Stellar laboratory
        static LEDGER_SEQUENCE: &u32 = &1000;
        const ID: &str = "0000004294967300098-0000000001";
        const PAGING_TOKEN: &str = "4294967300098-1";
        const ACCOUNT: &str = "GA7MC32ZYG5G7XSOR7TARZXXK5E4Y74VMWXIUZZNKIZ3Y3YQLCD25FV5";
        const RECORD_TYPE: &str = "account_created";
        const TYPE_I: u32 = 0;
        const CREATED_AT: &str = "2024-06-11T22:16:55Z";
        const STARTING_BALANCE: &str = "0.0000000";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let effects_for_ledger_request =
            EffectsForLedgerRequest::new().set_sequence(LEDGER_SEQUENCE);
        let effects_for_ledger_response = horizon_client
            .get_effects_for_ledger(&effects_for_ledger_request)
            .await;

        assert!(effects_for_ledger_response.is_ok());
        let binding = effects_for_ledger_response.clone().unwrap();
        let record = &binding.embedded().records()[0];

        assert_eq!(
            record.id,
            ID);

        assert_eq!(
            record.paging_token,
            PAGING_TOKEN);

        assert_eq!(
            record.account,
            ACCOUNT);

        assert_eq!(
            record.effect_type,
            RECORD_TYPE);

        assert_eq!(
            record.type_i,
            TYPE_I);

        assert_eq!(
            record.created_at,
            CREATED_AT);

        assert_eq!(
            record.starting_balance.as_ref().unwrap(),
            STARTING_BALANCE);
    }

    #[tokio::test]
    async fn test_get_effects_for_operation() {
        const OPERATION_ID: &str = "2314987376641";
        const ID: &str = "0000002314987376641-0000000001";
        const PAGING_TOKEN: &str = "2314987376641-1";
        const ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";
        const RECORD_TYPE: &str = "account_created";
        const TYPE_I: u32 = 0;
        const CREATED_AT: &str = "2024-06-11T21:36:12Z";
        const STARTING_BALANCE: &str = "10000000000.0000000";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let effects_for_operation_request = EffectsForOperationRequest::new()
            .set_operation_id(OPERATION_ID)
            .set_limit(2)
            .unwrap();
        let effects_for_operation_response = horizon_client
            .get_effects_for_operation(&effects_for_operation_request)
            .await;

        assert!(effects_for_operation_response.is_ok());

        let binding = effects_for_operation_response.clone().unwrap();
        let record = &binding.embedded().records()[0];

        assert_eq!(record.id(), ID);
        assert_eq!(record.paging_token(), PAGING_TOKEN);
        assert_eq!(record.account(), ACCOUNT);
        assert_eq!(record.effect_type(), RECORD_TYPE);
        assert_eq!(record.type_i(), &TYPE_I);
        assert_eq!(record.created_at(), CREATED_AT);
        assert_eq!(
            record.starting_balance().as_ref().unwrap(),
            &STARTING_BALANCE
        );
    }

    #[tokio::test]
    async fn test_get_effects_for_transaction() {
        const TRANSACTION_HASH: &str =
            "b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
        const ID: &str = "0000002314987376641-0000000001";
        const PAGING_TOKEN: &str = "2314987376641-1";
        const ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";
        const RECORD_TYPE: &str = "account_created";
        const TYPE_I: u32 = 0;
        const CREATED_AT: &str = "2024-06-11T21:36:12Z";
        const STARTING_BALANCE: &str = "10000000000.0000000";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let effects_for_transaction_request = EffectForTransactionRequest::new()
            .set_transaction_hash(TRANSACTION_HASH.to_string())
            .set_limit(2)
            .unwrap();

        let effects_for_transaction_response = horizon_client
            .get_effects_for_transaction(&effects_for_transaction_request)
            .await;

        assert!(effects_for_transaction_response.is_ok());

        let binding = effects_for_transaction_response.clone().unwrap();
        let record = &binding.embedded().records()[0];

        assert_eq!(record.id(), ID);
        assert_eq!(record.paging_token(), PAGING_TOKEN);
        assert_eq!(record.account(), ACCOUNT);
        assert_eq!(record.effect_type(), RECORD_TYPE);
        assert_eq!(record.type_i(), &TYPE_I);
        assert_eq!(record.created_at(), CREATED_AT);
        assert_eq!(
            record.starting_balance().as_ref().unwrap(),
            &STARTING_BALANCE
        );
    }
}
