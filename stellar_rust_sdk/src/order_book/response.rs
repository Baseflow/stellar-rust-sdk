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
    /// The prices and amounts for the buyside of the asset pair.
    pub bids: Vec<Bid>,
    /// The prices and amounts for the sellside of the asset pair.
    pub asks: Vec<Ask>,
    /// Details about the base asset.
    pub base: Base,
    /// Details about the counter asset.
    pub counter: Counter,
}

/// The prices and amounts for the buyside of the asset pair.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    #[serde(rename = "price_r")]
    /// A precise representation of the bid price of the asset pair.
    pub price_ratio: PriceR,
    /// The bid price of the base asset denominated in the counter asset. A number representing the decimal form of price_r.
    pub price: String,
    /// The amount of counter asset that the account making this offer is willing to buy at this price.
    pub amount: String,
}

/// A precise representation of the ask price of the asset pair.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct PriceR {
    /// The numenator.
    #[serde(rename = "n")]
    numenator: u32,
    /// The denominator.
    #[serde(rename = "d")]
    denominator: u32,
}

/// The prices and amounts for the sellside of the asset pair.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Ask {
    #[serde(rename = "price_r")]
    /// A precise representation of the ask price of the asset pair.
    pub price_ratio: PriceR,
    /// The ask price of the base asset denominated in the counter asset. A number representing the decimal form of
    pub price: String,
    /// The amount of counter asset that the account making this offer is willing to sell at this price.
    pub amount: String,
}

/// Details about the base asset.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Base {
    /// The type for the base asset. Either native, credit_alphanum4, or credit_alphanum12.
    #[serde(rename = "asset_type")]
    pub asset_type: Option<String>,
    /// The code for the base asset.
    #[serde(rename = "asset_code")]
    pub asset_code: Option<String>,
    /// The Stellar address of the base asset’s issuer.
    #[serde(rename = "asset_issuer")]
    pub asset_issuer: Option<String>,
}

/// Details about the counter asset.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Counter {
    /// The type for the counter asset. Either native, credit_alphanum4, or credit_alphanum12.
    #[serde(rename = "asset_type")]
    pub asset_type: Option<String>,
    /// The code for the counter asset.
    #[serde(rename = "asset_code")]
    pub asset_code: Option<String>,
    /// The Stellar address of the counter asset’s issuer.
    #[serde(rename = "asset_issuer")]
    pub asset_issuer: Option<String>,
}

impl Response for DetailsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let operation_record = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(operation_record)
    }
}

