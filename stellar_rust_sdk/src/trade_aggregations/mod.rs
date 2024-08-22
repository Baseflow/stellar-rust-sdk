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
    pub use super::response::*;
    pub use super::trade_aggregations_request::*;
}

#[cfg(test)]
pub mod test {
    use crate::{horizon_client::HorizonClient, trade_aggregations::prelude::*};

    // Request constants.
    const BASE_ASSET_ACCOUNT: &str = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";
    const BASE_ASSET_CODE: &str = "XETH";
    const COUNTER_ASSET_ACCOUNT: &str = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";
    const COUNTER_ASSET_CODE: &str = "XUSD";

    // Response constants.
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

    #[tokio::test]
    async fn test_set_offset() {
        // Create the base of a valid request which can be cloned by the individual tests.
        let request = TradeAggregationsRequest::new()
            .set_base_asset(AssetType::Alphanumeric4(AssetData {
                asset_issuer: BASE_ASSET_ACCOUNT.to_string(),
                asset_code: BASE_ASSET_CODE.to_string(),
            }))
            .unwrap()
            .set_counter_asset(AssetType::Alphanumeric4(AssetData {
                asset_issuer: COUNTER_ASSET_ACCOUNT.to_string(),
                asset_code: COUNTER_ASSET_CODE.to_string(),
            }))
            .unwrap();

        // Check if an error is returned when trying to set an offset, when the resolution is smaller than an hour.
        let result = request
            .clone()
            .set_resolution(Resolution(ResolutionData::Duration60000))
            .unwrap()
            .set_offset(60000);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Resolution must be greater than 1 hour when setting offset."
        );

        // Check if an error is returned when passing unwhole hours in milliseconds.
        let result = request
            .clone()
            .set_resolution(Resolution(ResolutionData::Duration604800000))
            .unwrap()
            .set_offset(3999999);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Offset must be in whole hours.");

        // Check if an error is returned if the offset is greater than the set resolution.
        let result = request
            .clone()
            .set_resolution(Resolution(ResolutionData::Duration3600000)) // 1 hour
            .unwrap()
            .set_offset(7200000); // 2 hours
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Offset must be smaller than the resolution."
        );

        // Check if an error is returned if the offset is greater than 24 hours.
        let result = request
            .clone()
            .set_resolution(Resolution(ResolutionData::Duration604800000))
            .unwrap()
            .set_offset(604800000); // 1 week
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Offset must be smaller than 24 hours.");
    }

    #[tokio::test]
    async fn test_get_trade_aggregations() {
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let trade_aggregations_request = TradeAggregationsRequest::new()
            .set_base_asset(AssetType::Alphanumeric4(AssetData {
                asset_issuer: BASE_ASSET_ACCOUNT.to_string(),
                asset_code: BASE_ASSET_CODE.to_string(),
            }))
            .unwrap()
            .set_counter_asset(AssetType::Alphanumeric4(AssetData {
                asset_issuer: COUNTER_ASSET_ACCOUNT.to_string(),
                asset_code: COUNTER_ASSET_CODE.to_string(),
            }))
            .unwrap()
            .set_resolution(Resolution(ResolutionData::Duration604800000))
            .unwrap();

        let trade_aggregations_response = horizon_client
            .get_trade_aggregations(&trade_aggregations_request)
            .await;

        // assert!(trade_aggregations_response.clone().is_ok());
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

    #[tokio::test]
    async fn test_asset_query_parameters() {
        use crate::models::*;
        // Test if different combinations of asset types result in a valid RESTful query. The `Native` asset, for example,
        // has a different amount of parameters than the alphanumeric types. The separators should always be correct, whatever
        // the combination.

        // Test 2 different, non-native, asset types.
        let request = TradeAggregationsRequest::new()
            .set_base_asset(AssetType::Alphanumeric4(AssetData {
                asset_issuer: "baseissuer".to_string(),
                asset_code: "basecode".to_string(),
            }))
            .unwrap()
            .set_counter_asset(AssetType::Alphanumeric12(AssetData {
                asset_issuer: "counterissuer".to_string(),
                asset_code: "countercode".to_string(),
            }))
            .unwrap()
            .set_resolution(Resolution(ResolutionData::Duration604800000))
            .unwrap();
        assert_eq!(request.get_query_parameters(),
            "?base_asset_type=credit_alphanum4&base_asset_code=basecode&base_asset_issuer=baseissuer&counter_asset_type=credit_alphanum12&counter_asset_code=countercode&counter_asset_issuer=counterissuer&resolution=604800000"
        );

        // Test 1 native, 1 non-native asset type.
        let request = TradeAggregationsRequest::new()
            .set_counter_asset(AssetType::Native)
            .unwrap()
            .set_base_asset(AssetType::Alphanumeric12(AssetData {
                asset_issuer: "counterissuer".to_string(),
                asset_code: "countercode".to_string(),
            }))
            .unwrap()
            .set_resolution(Resolution(ResolutionData::Duration604800000))
            .unwrap();
        assert_eq!(request.get_query_parameters(),
            "?base_asset_type=credit_alphanum12&base_asset_code=countercode&base_asset_issuer=counterissuer&counter_asset_type=native&resolution=604800000"
        );

        // Test 1 non-native, 1 native asset type.
        let request = TradeAggregationsRequest::new()
            .set_base_asset(AssetType::Alphanumeric4(AssetData {
                asset_issuer: "counterissuer".to_string(),
                asset_code: "countercode".to_string(),
            }))
            .unwrap()
            .set_resolution(Resolution(ResolutionData::Duration604800000))
            .unwrap()
            .set_counter_asset(AssetType::Native)
            .unwrap();
        assert_eq!(request.get_query_parameters(),
            "?base_asset_type=credit_alphanum4&base_asset_code=countercode&base_asset_issuer=counterissuer&counter_asset_type=native&resolution=604800000"
        );

        // Test 2 non-native asset types.
        let request = TradeAggregationsRequest::new()
            .set_base_asset(AssetType::Native)
            .unwrap()
            .set_resolution(Resolution(ResolutionData::Duration604800000))
            .unwrap()
            .set_counter_asset(AssetType::Native)
            .unwrap();
        assert_eq!(
            request.get_query_parameters(),
            "?base_asset_type=native&counter_asset_type=native&resolution=604800000"
        );
    }
}
