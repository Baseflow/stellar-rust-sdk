// TODO: documentation
pub mod all_trades_request;

// TODO: documentation
pub mod response;

// TODO: documentation
static TRADES_PATH: &str = "trades";

// TODO: documentation
pub mod prelude {
    pub use super::all_trades_request::*;
    pub use super::response::*;
}

// TODO: Write more tests.
#[cfg(test)]
pub mod test {
    use crate::{trades::prelude::*, horizon_client::HorizonClient};

    #[tokio::test]
    async fn create_request() {
        const LINK_SELF: &str = "";
        const LINK_BASE: &str = "https://horizon-testnet.stellar.org/accounts/GB4MMSZ5FY3KOCMMN77DNJBSKXFZVRXMLM5SKKDIVGTWGR55DKJM7GSD";
        const LINK_COUNTER: &str = "https://horizon-testnet.stellar.org/accounts/GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";
        const LINK_OPERATION: &str = "https://horizon-testnet.stellar.org/operations/4754528817153";
        const ID: &str = "4754528817153-0";
        const PAGING_TOKEN: &str = "4754528817153-0";
        const LEDGER_CLOSE_TIME: &str = "2024-02-06T19:10:19Z";
        const TRADE_TYPE: &str = "orderbook";
        const BASE_OFFER_ID: &str = "4611690772956205057";
        const BASE_ACCOUNT: &str = "GB4MMSZ5FY3KOCMMN77DNJBSKXFZVRXMLM5SKKDIVGTWGR55DKJM7GSD";
        const BASE_AMOUNT: &str = "3842030179.4600000";
        const BASE_ASSET_TYPE: &str = "credit_alphanum12";
        const BASE_ASSET_CODE: &str = "USDCAllow";
        const BASE_ASSET_ISSUER: &str = "GAWZGWFOURKXZ4XYXBGFADZM4QIG6BJNM74XIZCEIU3BHM62RN2MDEZN";
        const COUNTER_OFFER_ID: &str = "1";
        const COUNTER_ACCOUNT: &str = "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";
        const COUNTER_AMOUNT: &str = "3842030179.4600000";
        const COUNTER_ASSET_TYPE: &str = "credit_alphanum4";
        const COUNTER_ASSET_CODE: &str = "USDC";
        const COUNTER_ASSET_ISSUER: &str = "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";
        const BASE_IS_SELLER: &bool = &false;
        const PRICE_N: &str = "1";
        const PRICE_R: &str = "1";

        let all_trades_request = AllTradesRequest::new();
        
        let horizon_client =
        HorizonClient::new("https://horizon-testnet.stellar.org"
            .to_string())
            .unwrap();

        let all_trades_response = horizon_client
            .get_all_trades(&all_trades_request)
            .await;

        assert!(all_trades_response.clone().is_ok());
        let binding = all_trades_response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(response.links().self_link().href().as_ref().unwrap(), LINK_SELF);
        assert_eq!(response.links().base().href().as_ref().unwrap(), LINK_BASE);
        assert_eq!(response.links().counter().href().as_ref().unwrap(), LINK_COUNTER);
        assert_eq!(response.links().operation().href().as_ref().unwrap(), LINK_OPERATION);
        assert_eq!(response.id(), ID);
        assert_eq!(response.paging_token(), PAGING_TOKEN);
        assert_eq!(response.ledger_close_time(), LEDGER_CLOSE_TIME);
        assert_eq!(response.trade_type(), TRADE_TYPE);
        assert_eq!(response.base_offer_id(), BASE_OFFER_ID);
        assert_eq!(response.base_account(), BASE_ACCOUNT);
        assert_eq!(response.base_amount(), BASE_AMOUNT);
        assert_eq!(response.base_asset_type().as_ref().unwrap(), BASE_ASSET_TYPE);
        assert_eq!(response.base_asset_code().as_ref().unwrap(), BASE_ASSET_CODE);
        assert_eq!(response.base_asset_issuer().as_ref().unwrap(), BASE_ASSET_ISSUER);
        assert_eq!(response.counter_offer_id(), COUNTER_OFFER_ID);
        assert_eq!(response.counter_account(), COUNTER_ACCOUNT);
        assert_eq!(response.counter_amount(), COUNTER_AMOUNT);
        assert_eq!(response.counter_asset_type().as_ref().unwrap(), COUNTER_ASSET_TYPE);
        assert_eq!(response.counter_asset_code().as_ref().unwrap(), COUNTER_ASSET_CODE);
        assert_eq!(response.counter_asset_issuer().as_ref().unwrap(), COUNTER_ASSET_ISSUER);
        assert_eq!(response.base_is_seller(), BASE_IS_SELLER);
        assert_eq!(response.price().as_ref().unwrap().numenator(), PRICE_N);
        assert_eq!(response.price().as_ref().unwrap().denominator(), PRICE_R);
        }
}