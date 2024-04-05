use serde::{Deserialize, Serialize};

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
    pub asset_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Counter {
    #[serde(rename = "asset_type")]
    pub asset_type: String,
    #[serde(rename = "asset_code")]
    pub asset_code: String,
    #[serde(rename = "asset_issuer")]
    pub asset_issuer: String,
}
