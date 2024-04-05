use serde::{Deserialize, Serialize};

use crate::models::Response;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailsResponse {
    pub bids: Vec<Bid>,
    pub asks: Vec<Ask>,
    pub base: Base,
    pub counter: Counter,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    #[serde(rename = "price_r")]
    pub price_r: PriceR,
    pub price: String,
    pub amount: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceR {
    pub n: i64,
    pub d: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ask {
    #[serde(rename = "price_r")]
    pub price_r: PriceR,
    pub price: String,
    pub amount: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Base {
    #[serde(rename = "asset_type")]
    pub asset_type: Option<String>,
    #[serde(rename = "asset_code")]
    pub asset_code: Option<String>,
    #[serde(rename = "asset_issuer")]
    pub asset_issuer: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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