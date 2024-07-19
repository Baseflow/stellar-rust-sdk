use crate::models::prelude::*;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

// TODO: Documentation
// Explain the name `AllTradeAggregationsResponse` - it does not verbatim match the stellar docs, but does adhere to the 
/// naming conventions within this crate.
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

#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct TradeAggregationResponse {
    timestamp: String,
    trade_count: String,
    base_volume: String,
    counter_volume: String,
    avg: String,
    high: String,
    #[serde(rename = "high_r")]
    high_ratio: Ratio,
    low: String,
    #[serde(rename = "low_r")]
    low_ratio: Ratio,
    open: String,
    #[serde(rename = "open_r")]
    open_ratio: Ratio,
    close: String,
    #[serde(rename = "close_r")]
    close_ratio: Ratio,
}

impl Response for TradeAggregationResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}