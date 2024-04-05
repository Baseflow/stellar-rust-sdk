use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::models::Response;

/// Represents the response from the Horizon server when querying for the details of an order book.
/// 
/// This struct represents the response from the Horizon server when querying for the details of an order book.
/// It includes the bids, asks, base, and counter fields.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct DetailsResponse {
    /// The bids in the order book.
    pub bids: Vec<Bid>,
    /// The asks in the order book.
    pub asks: Vec<Ask>,
    /// The base asset of the order book.
    pub base: Base,
    /// The counter asset of the order book.
    pub counter: Counter,
}

/// Represents a bid in the order book.
/// 
/// This struct represents a bid in the order book. It includes the price_r, price, and amount fields.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    #[serde(rename = "price_r")]
    pub price_r: PriceR,
    pub price: String,
    pub amount: String,
}

/// Represents the price_r field in the order book.
/// 
/// This struct represents the price_r field in the order book. It includes the n and d fields.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct PriceR {
    pub n: i64,
    pub d: i64,
}

/// Represents an ask in the order book.
/// 
/// This struct represents an ask in the order book. It includes the price_r, price, and amount fields.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Ask {
    #[serde(rename = "price_r")]
    pub price_r: PriceR,
    pub price: String,
    pub amount: String,
}

/// Represents the base asset of the order book.
/// 
/// This struct represents the base asset of the order book. It includes the asset_type, asset_code, and asset_issuer fields.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Base {
    #[serde(rename = "asset_type")]
    pub asset_type: Option<String>,
    #[serde(rename = "asset_code")]
    pub asset_code: Option<String>,
    #[serde(rename = "asset_issuer")]
    pub asset_issuer: Option<String>,
}

/// Represents the counter asset of the order book.
/// 
/// This struct represents the counter asset of the order book. It includes the asset_type, asset_code, and asset_issuer fields.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Counter {
    #[serde(rename = "asset_type")]
    pub asset_type: Option<String>,
    #[serde(rename = "asset_code")]
    pub asset_code: Option<String>,
    #[serde(rename = "asset_issuer")]
    pub asset_issuer: Option<String>,
}

impl Response for DetailsResponse {
    fn from_json(json: String) -> Result<Self, String> {

        let operation_record = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(operation_record)
    }
}