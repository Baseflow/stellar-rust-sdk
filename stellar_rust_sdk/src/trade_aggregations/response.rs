use crate::models::prelude::*;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// Represents the response for the trade aggregations query in the Horizon API.
///
/// This struct defines the overall structure of the response for a trade aggregations query.
/// It includes navigational links and embedded results. 
///
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct AllTradeAggregationsResponse {
    #[serde(rename = "_links")]
    pub links: ResponseLinks,
    #[serde(rename = "_embedded")]
    pub embedded: Embedded<TradeAggregationResponse>,
}

impl Response for AllTradeAggregationsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}

/// Represents the precise buy and sell ratio of the trade.
///
/// This struct contains a numenator and a denominator, so that the trade ratio can be determined
/// in a precise manner.
///
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Ratio {
    /// The numenator.
    #[serde(rename = "n")]
    numenator: String,
    /// The denominator.
    #[serde(rename = "d")]
    denominator: String,
}

/// Represents a single record in a `TradeAggregations` query in the Horizon API.
///
/// This struct defines the overall structure of a record for a single trade aggregation.
/// It includes navigational links, a timestamp, a trade count, and additional data.
///
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct TradeAggregationResponse {
    // Start time for this trade aggregation. Represented as milliseconds since epoch.
    timestamp: String,
    // Total number of trades aggregated.
    trade_count: String,
    // Total volume of base asset.
    base_volume: String,
    // Total volume of counter asset.
    counter_volume: String,
    // Weighted average price of counter asset in terms of base asset.
    avg: String,
    // The highest price for this time period.
    high: String,
    // The highest price for this time period as a rational number.
    #[serde(rename = "high_r")]
    high_ratio: Ratio,
    // The lowest price for this time period.
    low: String,
    // The lowest price for this time period as a rational number.
    #[serde(rename = "low_r")]
    low_ratio: Ratio,
    // The price as seen on first trade aggregated.
    open: String,
    // The price as seen on first trade aggregated as a rational number.
    #[serde(rename = "open_r")]
    open_ratio: Ratio,
    // The price as seen on last trade aggregated.
    close: String,
    // The price as seen on last trade aggregated as a rational number.
    #[serde(rename = "close_r")]
    close_ratio: Ratio,
}

impl Response for TradeAggregationResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}