use all_liquidity_pools_request::AllLiquidityPoolsRequest;
use single_liquidity_pool_request::SingleLiquidityPoolRequest;

use crate::horizon_client::HorizonClient;

pub mod all_liquidity_pools_request;
pub mod response;
pub mod single_liquidity_pool_request;

static LIQUIDITY_POOLS_PATH: &str = "liquidity_pools";

pub mod prelude {
    pub use super::all_liquidity_pools_request::*;
    pub use super::response::*;
    pub use super::single_liquidity_pool_request::*;
}

#[tokio::test]
async fn test_get_all_liquidity_pools() {
    const LIQUIDITY_POOL_ID: &str =
        "4cd1f6defba237eecbc5fefe259f89ebc4b5edd49116beb5536c4034fc48d63f";
    const LIQUIDITY_POOL_PAGING_TOKEN: &str =
        "4cd1f6defba237eecbc5fefe259f89ebc4b5edd49116beb5536c4034fc48d63f";
    const LIQUIDITY_POOL_FEE_BP: i64 = 30;
    const LIQUIDITY_POOL_TYPE: &str = "constant_product";
    const LIQUIDITY_POOL_TOTAL_TRUSTLINES: &str = "1";
    const LIQUIDITY_POOL_RESERVE_ASSET_0: &str = "native";
    const LIQUIDITY_POOL_RESERVE_ASSET_1: &str =
        "USDC:GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";
    const LIQUIDITY_POOL_RESERVE_AMOUNT_1: &str = "200.0267182";
    const LIQUIDITY_POOL_LAST_MODIFIED_LEDGER: i64 = 555798;
    const LIQUIDITY_POOL_LAST_MODIFIED_TIME: &str = "2024-03-11T12:58:38Z";

    let horizon_client =
        HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

    let all_liquidity_pools_request = AllLiquidityPoolsRequest::new()
        .add_native_reserve()
        .add_alphanumeric4_reserve(
            "USDC".to_string(),
            "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5".to_string(),
        )
        .set_limit(2);

    let all_liquidity_pools_response = horizon_client
        .get_all_liquidity_pools(&all_liquidity_pools_request)
        .await;

    assert!(all_liquidity_pools_response.clone().is_ok());

    let binding = all_liquidity_pools_response.unwrap();
    let all_liquidity_pools_response = &binding.embedded().records()[0];

    assert_eq!(all_liquidity_pools_response.id(), LIQUIDITY_POOL_ID);
    assert_eq!(
        all_liquidity_pools_response.paging_token(),
        LIQUIDITY_POOL_PAGING_TOKEN
    );
    assert_eq!(
        all_liquidity_pools_response.fee_bp(),
        &LIQUIDITY_POOL_FEE_BP
    );
    assert_eq!(
        all_liquidity_pools_response.type_field(),
        LIQUIDITY_POOL_TYPE
    );
    assert_eq!(
        all_liquidity_pools_response.total_trustlines(),
        LIQUIDITY_POOL_TOTAL_TRUSTLINES
    );
    assert_eq!(
        all_liquidity_pools_response.reserves()[0].asset(),
        LIQUIDITY_POOL_RESERVE_ASSET_0
    );
    assert_eq!(
        all_liquidity_pools_response.reserves()[1].asset(),
        LIQUIDITY_POOL_RESERVE_ASSET_1
    );
}

#[tokio::test]
async fn test_get_single_liquidity_pool() {
    const LIQUIDITY_POOL_ID: &str =
        "01c58ab8fb283c8b083a26bf2fe06b7b6c6304c13f9d29d956cdf15a48bea72d";
    const LIQUIDITY_POOL_PAGING_TOKEN: &str =
        "01c58ab8fb283c8b083a26bf2fe06b7b6c6304c13f9d29d956cdf15a48bea72d";
    const LIQUIDITY_POOL_FEE_BP: i64 = 30;
    const LIQUIDITY_POOL_TYPE: &str = "constant_product";
    const LIQUIDITY_POOL_TOTAL_TRUSTLINES: &str = "1";
    const LIQUIDITY_POOL_TOTAL_SHARES: &str = "150.0000000";
    const LIQUIDITY_POOL_RESERVE_ASSET_0: &str =
        "SDK:GB7IIVQLLJ3AY3DWSCACVJBZL7FFKDK4D3PMHPBBAIHPVVB3BZYUG5UN";
    const LIQUIDITY_POOL_RESERVE_AMOUNT_0: &str = "160.0000000";
    const LIQUIDITY_POOL_RESERVE_ASSET_1: &str =
        "PHPSTAR:GAJHVDRVJHC2ORAERVVQYWADMEJLUX6H2SZ5MMEPI7BUHJYSTTFRZW7W";
    const LIQUIDITY_POOL_RESERVE_AMOUNT_1: &str = "140.6513722";
    const LIQUIDITY_POOL_LAST_MODIFIED_LEDGER: i64 = 249832;
    const LIQUIDITY_POOL_LAST_MODIFIED_TIME: &str = "2024-02-21T22:18:27Z";

    let horizon_client =
        HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

    let single_liquidity_pool_request = SingleLiquidityPoolRequest::new()
        .set_liquidity_pool_id(
            "01c58ab8fb283c8b083a26bf2fe06b7b6c6304c13f9d29d956cdf15a48bea72d".to_string(),
        )
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
