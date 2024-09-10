/// Provides the `SingleLiquidityPoolRequest`.
///
/// This module provides the `SingleLiquidityPoolRequest` struct, specifically designed for
/// constructing requests to query information about a single liquidity pool from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_single_liquidity_pool`](crate::horizon_client::HorizonClient::get_single_liquidity_pool)
/// method.
///
pub mod single_liquidity_pool_request;

/// Provides the `AllLiquidityPoolsRequest`.
///
/// This module provides the `AllLiquidityPoolsRequest` struct, specifically designed for
/// constructing requests to query information about all liquidity pools from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_all_liquidity_pools`](crate::horizon_client::HorizonClient::get_all_liquidity_pools)
/// method.
pub mod all_liquidity_pools_request;

/// Provides the responses.
///
/// This module defines structures representing the response from the Horizon API when querying
/// for liquidity pools. The structures are designed to deserialize the JSON response into Rust
/// objects, enabling straightforward access to various details of a single Stellar account.
///
/// These structures are equipped with serialization capabilities to handle the JSON data from the
/// Horizon server and with getter methods for easy field access.
///
pub mod response;

/// The base path for liquidity pool related endpoints in the Horizon API.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for liquidity pool related API calls.
///
pub(crate) static LIQUIDITY_POOLS_PATH: &str = "liquidity_pools";

/// The `prelude` module of the `liquidity_pools` module.
///
/// This module serves as a convenience for users of the Horizon Rust SDK, allowing for easy and
/// ergonomic import of the most commonly used items across various modules. It re-exports
/// key structs and traits from the sibling modules, simplifying access to these components
/// when using the library.
///
/// By importing the contents of `prelude`, users can conveniently access the primary
/// functionalities of the liquidity pool related modules without needing to import each item
/// individually.
///
/// # Contents
///
/// The `prelude` includes the following re-exports:
///
/// * From `single_liquidity_pool_request`: All items (e.g. `SingleLiquidityPoolRequest`).
/// * From `all_liquidity_pools_request`: All items (e.g. `AllLiquidityPoolsRequest`, `Reserve`, etc.).
/// * From `response`: All items (e.g. `AllLiquidityPoolsResponse`, `Reserve`, etc.).
///
/// # Example
/// ```
/// # use crate::stellar_rs::models::*;
/// // Import the contents of the liquidity pools prelude
/// use stellar_rs::liquidity_pools::prelude::*;
///
/// // Now you can directly use SingleLiquidityPoolRequest, LiquidityPool, etc.
/// let single_lp_request = SingleLiquidityPoolRequest::new();
/// ```
///
pub mod prelude {
    pub use super::response::*;
    pub use super::single_liquidity_pool_request::*;
}

#[tokio::test]
async fn test_get_all_liquidity_pools() {
    use crate::horizon_client::HorizonClient;
    use all_liquidity_pools_request::AllLiquidityPoolsRequest;

    const RSP_1_LIQUIDITY_POOL_ID: &str =
        "4cd1f6defba237eecbc5fefe259f89ebc4b5edd49116beb5536c4034fc48d63f";
    const RSP_1_LIQUIDITY_POOL_PAGING_TOKEN: &str =
        "4cd1f6defba237eecbc5fefe259f89ebc4b5edd49116beb5536c4034fc48d63f";
    const RSP_1_LIQUIDITY_POOL_FEE_BP: i64 = 30;
    const RSP_1_LIQUIDITY_POOL_TYPE: &str = "constant_product";
    const RSP_1_LIQUIDITY_POOL_TOTAL_TRUSTLINES: &str = "2";
    const RSP_1_LIQUIDITY_POOL_RESERVE_ASSET_0: &str = "native";
    const RSP_1_LIQUIDITY_POOL_RESERVE_ASSET_1: &str =
        "USDC:GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";

    const RSP_2_LIQUIDITY_POOL_ID: &str =
        "03eac63a834b31201652ed575c816b4e7beb0c4eece47caf956ff83648f486d0";
    const RSP_2_LIQUIDITY_POOL_PAGING_TOKEN: &str =
        "03eac63a834b31201652ed575c816b4e7beb0c4eece47caf956ff83648f486d0";
    const RSP_2_LIQUIDITY_POOL_FEE_BP: i64 = 30;
    const RSP_2_LIQUIDITY_POOL_TYPE: &str = "constant_product";
    const RSP_2_LIQUIDITY_POOL_TOTAL_TRUSTLINES: &str = "1";
    const RSP_2_LIQUIDITY_POOL_TOTAL_SHARES: &str = "249.0000000";
    const RSP_2_LIQUIDITY_POOL_RESERVE_ASSET_0: &str = "native";
    const RSP_2_LIQUIDITY_POOL_RESERVE_AMOUNT_0: &str = "2939.0000000";
    const RSP_2_LIQUIDITY_POOL_RESERVE_ASSET_1: &str =
        "FLUTTER:GCGTOQSNERFVVJ6Y7YZYDF3MTZIY63KIEFMKA26Q7YPV3AFYD2JSRNYN";
    const RSP_2_LIQUIDITY_POOL_RESERVE_AMOUNT_1: &str = "21.1917647";

    const RSP_3_LIQUIDITY_POOL_ID: &str =
        "0b3c88caa5aeada296646c1810893e3b04cba0426cff8ff6a63cf6f35cc7f5b3";
    const RSP_3_LIQUIDITY_POOL_PAGING_TOKEN: &str =
        "0b3c88caa5aeada296646c1810893e3b04cba0426cff8ff6a63cf6f35cc7f5b3";
    const RSP_3_LIQUIDITY_POOL_FEE_BP: i64 = 30;
    const RSP_3_LIQUIDITY_POOL_TYPE: &str = "constant_product";
    const RSP_3_LIQUIDITY_POOL_TOTAL_TRUSTLINES: &str = "1";
    const RSP_3_LIQUIDITY_POOL_TOTAL_SHARES: &str = "150.0000000";
    const RSP_3_LIQUIDITY_POOL_RESERVE_ASSET_0: &str =
        "SDK:GAGTRBIF75N7NUA37JGGJZKXIS4JJKTQERRFWTP5DN4SM4OC2T6QPMQB";
    const RSP_3_LIQUIDITY_POOL_RESERVE_AMOUNT_0: &str = "158.0667366";
    const RSP_3_LIQUIDITY_POOL_RESERVE_ASSET_1: &str =
        "FLUTTER:GCGTOQSNERFVVJ6Y7YZYDF3MTZIY63KIEFMKA26Q7YPV3AFYD2JSRNYN";
    const RSP_3_LIQUIDITY_POOL_RESERVE_AMOUNT_1: &str = "142.3768102";

    let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org").unwrap();

    let all_liquidity_pools_request_1 = AllLiquidityPoolsRequest::new()
        .add_native_reserve()
        .add_alphanumeric4_reserve(
            "USDC",
            "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5",
        )
        .set_limit(2)
        .unwrap();

    let all_liquidity_pools_response_1 = horizon_client
        .get_all_liquidity_pools(&all_liquidity_pools_request_1)
        .await;

    assert!(all_liquidity_pools_response_1.clone().is_ok());

    let binding = all_liquidity_pools_response_1.unwrap();
    let all_liquidity_pools_response_2 = &binding.embedded().records()[0];

    assert_eq!(all_liquidity_pools_response_2.id(), RSP_1_LIQUIDITY_POOL_ID);
    assert_eq!(
        all_liquidity_pools_response_2.paging_token(),
        RSP_1_LIQUIDITY_POOL_PAGING_TOKEN
    );
    assert_eq!(
        all_liquidity_pools_response_2.fee_bp(),
        &RSP_1_LIQUIDITY_POOL_FEE_BP
    );
    assert_eq!(
        all_liquidity_pools_response_2.type_field(),
        RSP_1_LIQUIDITY_POOL_TYPE
    );
    assert_eq!(
        all_liquidity_pools_response_2.total_trustlines(),
        RSP_1_LIQUIDITY_POOL_TOTAL_TRUSTLINES
    );
    assert_eq!(
        all_liquidity_pools_response_2.reserves()[0].asset(),
        RSP_1_LIQUIDITY_POOL_RESERVE_ASSET_0
    );
    assert!(!all_liquidity_pools_response_2.reserves()[0]
        .amount()
        .is_empty());
    assert_eq!(
        all_liquidity_pools_response_2.reserves()[1].asset(),
        RSP_1_LIQUIDITY_POOL_RESERVE_ASSET_1
    );
    assert!(!all_liquidity_pools_response_2.reserves()[1]
        .amount()
        .is_empty());

    let all_liquidity_pools_request_2 = AllLiquidityPoolsRequest::new()
        .add_native_reserve()
        .add_alphanumeric12_reserve(
            "FLUTTER",
            "GCGTOQSNERFVVJ6Y7YZYDF3MTZIY63KIEFMKA26Q7YPV3AFYD2JSRNYN",
        )
        .set_limit(2)
        .unwrap();

    let all_liquidity_pools_response_2 = horizon_client
        .get_all_liquidity_pools(&all_liquidity_pools_request_2)
        .await;

    assert!(all_liquidity_pools_response_2.clone().is_ok());

    let binding = all_liquidity_pools_response_2.unwrap();
    let all_liquidity_pools_response_2 = &binding.embedded().records()[0];

    assert_eq!(all_liquidity_pools_response_2.id(), RSP_2_LIQUIDITY_POOL_ID);
    assert_eq!(
        all_liquidity_pools_response_2.paging_token(),
        RSP_2_LIQUIDITY_POOL_PAGING_TOKEN
    );
    assert_eq!(
        all_liquidity_pools_response_2.fee_bp(),
        &RSP_2_LIQUIDITY_POOL_FEE_BP
    );
    assert_eq!(
        all_liquidity_pools_response_2.type_field(),
        RSP_2_LIQUIDITY_POOL_TYPE
    );
    assert_eq!(
        all_liquidity_pools_response_2.total_trustlines(),
        RSP_2_LIQUIDITY_POOL_TOTAL_TRUSTLINES
    );
    assert_eq!(
        all_liquidity_pools_response_2.total_shares(),
        RSP_2_LIQUIDITY_POOL_TOTAL_SHARES
    );
    assert_eq!(
        all_liquidity_pools_response_2.reserves()[0].asset(),
        RSP_2_LIQUIDITY_POOL_RESERVE_ASSET_0
    );
    assert_eq!(
        all_liquidity_pools_response_2.reserves()[0].amount(),
        RSP_2_LIQUIDITY_POOL_RESERVE_AMOUNT_0
    );
    assert_eq!(
        all_liquidity_pools_response_2.reserves()[1].asset(),
        RSP_2_LIQUIDITY_POOL_RESERVE_ASSET_1
    );
    assert_eq!(
        all_liquidity_pools_response_2.reserves()[1].amount(),
        RSP_2_LIQUIDITY_POOL_RESERVE_AMOUNT_1
    );

    let all_liquidity_pools_request_3 = AllLiquidityPoolsRequest::new()
        .add_alphanumeric4_reserve(
            "SDK",
            "GAGTRBIF75N7NUA37JGGJZKXIS4JJKTQERRFWTP5DN4SM4OC2T6QPMQB",
        )
        .set_limit(2)
        .unwrap();

    let all_liquidity_pools_response_3 = horizon_client
        .get_all_liquidity_pools(&all_liquidity_pools_request_3)
        .await;

    assert!(all_liquidity_pools_response_3.clone().is_ok());

    let binding = all_liquidity_pools_response_3.unwrap();
    let all_liquidity_pools_response_3 = &binding.embedded().records()[0];

    assert_eq!(all_liquidity_pools_response_3.id(), RSP_3_LIQUIDITY_POOL_ID);
    assert_eq!(
        all_liquidity_pools_response_3.paging_token(),
        RSP_3_LIQUIDITY_POOL_PAGING_TOKEN
    );
    assert_eq!(
        all_liquidity_pools_response_3.fee_bp(),
        &RSP_3_LIQUIDITY_POOL_FEE_BP
    );
    assert_eq!(
        all_liquidity_pools_response_3.type_field(),
        RSP_3_LIQUIDITY_POOL_TYPE
    );
    assert_eq!(
        all_liquidity_pools_response_3.total_trustlines(),
        RSP_3_LIQUIDITY_POOL_TOTAL_TRUSTLINES
    );
    assert_eq!(
        all_liquidity_pools_response_3.total_shares(),
        RSP_3_LIQUIDITY_POOL_TOTAL_SHARES
    );
    assert_eq!(
        all_liquidity_pools_response_3.reserves()[0].asset(),
        RSP_3_LIQUIDITY_POOL_RESERVE_ASSET_0
    );
    assert_eq!(
        all_liquidity_pools_response_3.reserves()[0].amount(),
        RSP_3_LIQUIDITY_POOL_RESERVE_AMOUNT_0
    );
    assert_eq!(
        all_liquidity_pools_response_3.reserves()[1].asset(),
        RSP_3_LIQUIDITY_POOL_RESERVE_ASSET_1
    );
    assert_eq!(
        all_liquidity_pools_response_3.reserves()[1].amount(),
        RSP_3_LIQUIDITY_POOL_RESERVE_AMOUNT_1
    );
}

#[tokio::test]
async fn test_get_single_liquidity_pool() {
    use crate::horizon_client::HorizonClient;
    use single_liquidity_pool_request::SingleLiquidityPoolRequest;

    const LIQUIDITY_POOL_ID: &str =
        "03eac63a834b31201652ed575c816b4e7beb0c4eece47caf956ff83648f486d0";
    const LIQUIDITY_POOL_PAGING_TOKEN: &str =
        "03eac63a834b31201652ed575c816b4e7beb0c4eece47caf956ff83648f486d0";
    const LIQUIDITY_POOL_FEE_BP: i64 = 30;
    const LIQUIDITY_POOL_TYPE: &str = "constant_product";
    const LIQUIDITY_POOL_TOTAL_TRUSTLINES: &str = "1";
    const LIQUIDITY_POOL_TOTAL_SHARES: &str = "249.0000000";
    const LIQUIDITY_POOL_RESERVE_ASSET_0: &str = "native";
    const LIQUIDITY_POOL_RESERVE_AMOUNT_0: &str = "2939.0000000";
    const LIQUIDITY_POOL_RESERVE_ASSET_1: &str =
        "FLUTTER:GCGTOQSNERFVVJ6Y7YZYDF3MTZIY63KIEFMKA26Q7YPV3AFYD2JSRNYN";
    const LIQUIDITY_POOL_RESERVE_AMOUNT_1: &str = "21.1917647";
    const LIQUIDITY_POOL_LAST_MODIFIED_LEDGER: i64 = 1025861;
    const LIQUIDITY_POOL_LAST_MODIFIED_TIME: &str = "2024-08-13T07:20:55Z";

    let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org").unwrap();

    let single_liquidity_pool_request = SingleLiquidityPoolRequest::new()
        .set_liquidity_pool_id("03eac63a834b31201652ed575c816b4e7beb0c4eece47caf956ff83648f486d0")
        .unwrap();

    let single_liquidity_pool_response = horizon_client
        .get_single_liquidity_pool(&single_liquidity_pool_request)
        .await;

    assert!(single_liquidity_pool_response.clone().is_ok());

    let single_liquidity_pool_response = single_liquidity_pool_response.unwrap();

    assert_eq!(single_liquidity_pool_response.id(), LIQUIDITY_POOL_ID);
    assert_eq!(
        single_liquidity_pool_response.paging_token(),
        LIQUIDITY_POOL_PAGING_TOKEN
    );
    assert_eq!(
        single_liquidity_pool_response.fee_bp(),
        &LIQUIDITY_POOL_FEE_BP
    );
    assert_eq!(
        single_liquidity_pool_response.type_field(),
        LIQUIDITY_POOL_TYPE
    );
    assert_eq!(
        single_liquidity_pool_response.total_trustlines(),
        LIQUIDITY_POOL_TOTAL_TRUSTLINES
    );
    assert_eq!(
        single_liquidity_pool_response.total_shares(),
        LIQUIDITY_POOL_TOTAL_SHARES
    );
    assert_eq!(
        single_liquidity_pool_response.reserves()[0].asset(),
        LIQUIDITY_POOL_RESERVE_ASSET_0
    );
    assert_eq!(
        single_liquidity_pool_response.reserves()[0].amount(),
        LIQUIDITY_POOL_RESERVE_AMOUNT_0
    );
    assert_eq!(
        single_liquidity_pool_response.reserves()[1].asset(),
        LIQUIDITY_POOL_RESERVE_ASSET_1
    );
    assert_eq!(
        single_liquidity_pool_response.reserves()[1].amount(),
        LIQUIDITY_POOL_RESERVE_AMOUNT_1
    );
    assert_eq!(
        single_liquidity_pool_response.last_modified_ledger(),
        &LIQUIDITY_POOL_LAST_MODIFIED_LEDGER
    );
    assert_eq!(
        single_liquidity_pool_response.last_modified_time(),
        LIQUIDITY_POOL_LAST_MODIFIED_TIME
    );
}
