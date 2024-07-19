// TODO: Documentation
pub mod trade_aggregations_request;

// TODO: Documentation
pub mod response;

// TODO: Documentation
static TRADE_AGGREGATIONS_PATH: &str = "trade_aggregations";

// TODO: Documentation
pub mod prelude {
    pub use super::trade_aggregations_request::*;
    pub use super::response::*;
}

#[cfg(test)]
pub mod test {
    use crate::{trade_aggregations::prelude::*, horizon_client::HorizonClient};

    #[tokio::test]
    async fn test_get_trade_aggregations() {
        // Request constants
        const BASE_ASSET_ACCOUNT: &str = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";
        const BASE_ASSET_CODE: &str = "XETH";
        const COUNTER_ASSET_ACCOUNT: &str = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";
        const COUNTER_ASSET_CODE: &str = "XUSD";
        const RESOLUTION: &str = &"604800000";
        // Response constants
        const TIMESTAMP: &str = "1717632000000";
        const TRADE_COUNT: &str = "39";
        const BASE_VOLUME: &str = "66.7280000";
        const COUNTER_VOLUME: &str = "51.0800000";
        const AVG: &str = "0.7654957";
        const HIGH: &str = "10.0000000";
        const HIGH_N: &str = "10";
        const HIGH_D: &str = "1";
        const LOW: &str = "0.1000000";
        const LOW_N: &str = "1";
        const LOW_D: &str = "10";
        const OPEN: &str = "0.3000000";
        const OPEN_N: &str = "3";
        const OPEN_D: &str = "10";
        const CLOSE: &str = "10.0000000";
        const CLOSE_N: &str = "10";
        const CLOSE_D: &str = "1";
        
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org"
                .to_string())
                .unwrap();
        
        let trade_aggregations_request = TradeAggregationsRequest::new()
            .set_base_asset(AssetType::Alphanumeric4(AssetData{
                asset_issuer: BASE_ASSET_ACCOUNT.to_string(),
                asset_code: BASE_ASSET_CODE.to_string(),
            }))
            .unwrap()
            .set_counter_asset(AssetType::Alphanumeric4(AssetData{
                asset_issuer: COUNTER_ASSET_ACCOUNT.to_string(),
                asset_code: COUNTER_ASSET_CODE.to_string(),
            }))
            .unwrap()
            .set_resolution(RESOLUTION.to_string())
            .unwrap();

        let trade_aggregations_response = horizon_client
            .get_trade_aggregations(&trade_aggregations_request)
            .await;

        assert!(trade_aggregations_response.clone().is_ok());
        let binding = trade_aggregations_response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(response.timestamp(), TIMESTAMP);
        assert_eq!(response.trade_count(), TRADE_COUNT);
        assert_eq!(response.base_volume(), BASE_VOLUME);
        assert_eq!(response.counter_volume(), COUNTER_VOLUME);
        assert_eq!(response.avg(), AVG);
        assert_eq!(response.high(), HIGH);
        assert_eq!(response.high_ratio().numenator(), HIGH_N);
        assert_eq!(response.high_ratio().denominator(), HIGH_D);
        assert_eq!(response.low(), LOW);
        assert_eq!(response.low_ratio().numenator(), LOW_N);
        assert_eq!(response.low_ratio().denominator(), LOW_D);
        assert_eq!(response.open(), OPEN);
        assert_eq!(response.open_ratio().numenator(), OPEN_N);
        assert_eq!(response.open_ratio().denominator(), OPEN_D);
        assert_eq!(response.close(), CLOSE);
        assert_eq!(response.close_ratio().numenator(), CLOSE_N);
        assert_eq!(response.close_ratio().denominator(), CLOSE_D);
    }
}