use crate::models::prelude::*;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

// TODO: Documentation.
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct AllTradesResponse {
    #[serde(rename = "_links")]
    pub links: ResponseLinks,
    #[serde(rename = "_embedded")]
    pub embedded: Embedded<TradeResponse>,
}

impl Response for AllTradesResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}

// TODO: Documentation.
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Links {
    #[serde(rename = "self")]
    self_link: Link,
    base: Link,
    counter: Link,
    operation: Link,
}

/// Represents the precise buy and sell price of the assets on offer.
///
/// This struct contains a numenator and a denominator, so that the price ratio can be determined
/// in a precise manner.
///
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct Price {
    /// The numenator.
    #[serde(rename = "n")]
    numenator: String,
    /// The denominator.
    #[serde(rename = "d")]
    denominator: String,
}

// TODO: Documentation and descriptions of fields.
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct TradeResponse {
    #[serde(rename = "_links")]
    links: Links,
    id: String,
    paging_token: String,
    ledger_close_time: String,
    trade_type: String,
    liquidity_pool_fee_bp: Option<u32>,
    base_liquidity_pool_id: Option<String>,
    base_offer_id: String,
    base_account: String,
    base_amount: String,
    base_asset_type: Option<String>,
    base_asset_code: Option<String>,
    base_asset_issuer: Option<String>,
    counter_liquidity_pool_id: Option<String>,
    counter_offer_id: String,
    counter_account: String,
    counter_amount: String,
    counter_asset_type: Option<String>,
    counter_asset_code: Option<String>,
    counter_asset_issuer: Option<String>,
    base_is_seller: bool,
    price: Option<Price>,
}

impl Response for TradeResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}