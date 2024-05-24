pub mod all_liquidity_pools_request;
pub mod response;
pub mod single_liquidity_pool_request;

static LIQUIDITY_POOLS_PATH: &str = "liquidity_pools";

pub mod prelude {
    pub use super::response::*;
    pub use super::single_liquidity_pool_request::*;
}

#[tokio::test]
async fn test_get_all_liquidity_pools() {
    use crate::{horizon_client::HorizonClient, Paginatable};
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
        "0a9a1af472bd6107075610add5759bddfb1d82f80c664ee5198cd24509541841";
    const RSP_2_LIQUIDITY_POOL_PAGING_TOKEN: &str =
        "0a9a1af472bd6107075610add5759bddfb1d82f80c664ee5198cd24509541841";
    const RSP_2_LIQUIDITY_POOL_FEE_BP: i64 = 30;
    const RSP_2_LIQUIDITY_POOL_TYPE: &str = "constant_product";
    const RSP_2_LIQUIDITY_POOL_TOTAL_TRUSTLINES: &str = "1";
    const RSP_2_LIQUIDITY_POOL_TOTAL_SHARES: &str = "0.0000000";
    const RSP_2_LIQUIDITY_POOL_RESERVE_ASSET_0: &str = "native";
    const RSP_2_LIQUIDITY_POOL_RESERVE_AMOUNT_0: &str = "0.0000000";
    const RSP_2_LIQUIDITY_POOL_RESERVE_ASSET_1: &str =
        "NOODLE:GARPXPHGABTN52WPJ2QZQBY7TSXMK7PXLKOD6FSPA2TEVSJDWIDRSHPO";
    const RSP_2_LIQUIDITY_POOL_RESERVE_AMOUNT_1: &str = "0.0000000";

    const RSP_3_LIQUIDITY_POOL_ID: &str =
        "02ea143d8ac56fa6e936c3558edfce6c64b7a223827a006805b279104bac2851";
    const RSP_3_LIQUIDITY_POOL_PAGING_TOKEN: &str =
        "02ea143d8ac56fa6e936c3558edfce6c64b7a223827a006805b279104bac2851";
    const RSP_3_LIQUIDITY_POOL_FEE_BP: i64 = 30;
    const RSP_3_LIQUIDITY_POOL_TYPE: &str = "constant_product";
    const RSP_3_LIQUIDITY_POOL_TOTAL_TRUSTLINES: &str = "2";
    const RSP_3_LIQUIDITY_POOL_TOTAL_SHARES: &str = "5196.1524225";
    const RSP_3_LIQUIDITY_POOL_RESERVE_ASSET_0: &str =
        "LPA7:GDTCZZNMT74SEGPDQL3IQJDJ54MXQJHSNQ3HNNJJIJLIVG2LOSDLG2OZ";
    const RSP_3_LIQUIDITY_POOL_RESERVE_AMOUNT_0: &str = "3000.0000000";
    const RSP_3_LIQUIDITY_POOL_RESERVE_ASSET_1: &str =
        "LPB7:GDTBT5GAABYAENDWYXB2IAHYXMWDGK2ZG5H4ZDQL32DR7KS6L7NGC5I3";
    const RSP_3_LIQUIDITY_POOL_RESERVE_AMOUNT_1: &str = "9000.0000000";

    let horizon_client =
        HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

    let all_liquidity_pools_request_1 = AllLiquidityPoolsRequest::new()
        .add_native_reserve()
        .add_alphanumeric4_reserve(
            "USDC".to_string(),
            "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5".to_string(),
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
            "NOODLE".to_string(),
            "GARPXPHGABTN52WPJ2QZQBY7TSXMK7PXLKOD6FSPA2TEVSJDWIDRSHPO".to_string(),
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
            "LPA7".to_string(),
            "GDTCZZNMT74SEGPDQL3IQJDJ54MXQJHSNQ3HNNJJIJLIVG2LOSDLG2OZ".to_string(),
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
