/// Provides the `TradeAggregationsRequest`.
///
/// This module provides the `TradeAggregationsRequest` struct, specifically designed for
/// constructing requests to query information about trade aggregations from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_trade_aggregations`](crate::horizon_client::HorizonClient::get_trade_aggregations)
/// method.
///
pub mod trade_aggregations_request;

/// Provides the response.
///
/// This module defines structures representing the response from the Horizon API when querying
/// for trade aggregations. The structures are designed to deserialize the JSON response into Rust
/// objects, enabling straightforward access to various details of a single Stellar account.
///
/// These structures are equipped with serialization capabilities to handle the JSON data from the
/// Horizon server and with getter methods for easy field access.
///
pub mod response;

/// The base path for trade aggregations related endpoints in the Horizon API.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for trade aggregations related API calls.
///
static TRADE_AGGREGATIONS_PATH: &str = "trade_aggregations";

/// The `prelude` module of the `trade aggregations` module.
///
/// This module serves as a convenience for users of the Horizon Rust SDK, allowing for easy and
/// ergonomic import of the most commonly used items across various modules. It re-exports
/// key structs and traits from the sibling modules, simplifying access to these components
/// when using the library.
///
/// By importing the contents of `prelude`, users can conveniently access the primary
/// functionalities of the trade aggregations related modules without needing to import each item
/// individually.
///
/// # Contents
///
/// The `prelude` includes the following re-exports:
///
/// * From `trade_aggregations_request`: All items (e.g. `TradeAggregationsRequest`, `Resolution`, etc.).
/// * From `response`: All items (e.g. `AllTradeAggregationsResponse`, `TradeAggregationResponse`, etc.).
///
/// # Example
/// ```
/// # use crate::stellar_rs::models::*;
/// // Import the contents of the offers prelude
/// use stellar_rs::trade_aggregations::prelude::*;
///
/// // Now you can directly use TradeAggregationsRequest.
/// let trade_aggregations_request = TradeAggregationsRequest::new();
/// ```
///
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
            .set_resolution(Resolution(ResolutionData::Value604800000))
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