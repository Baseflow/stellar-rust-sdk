use crate::models::prelude::*;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// Represents the response for the 'all trades' query in the Horizon API.
///
/// This struct defines the overall structure of the response for an 'all trades' query.
/// It includes navigational links and embedded results.
///
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

/// Represents the navigational links in a single trade response from the Horizon API.
///
/// This struct includes various hyperlinks such as links to the trade itself,
/// base asset, counter asset and operation.
///
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct TradeResponseLinks {
    #[serde(rename = "self")]
    self_link: Link,
    base: Link,
    counter: Link,
    operation: Link,
}

/// Represents the precise buy and sell price of the trade.
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

/// Represents the response for a single trade query in the Horizon API.
///
/// This struct defines the overall structure of the response for a single trade query.
/// It includes navigational links, the id, the base asset, the counter asset, and additional data.
///
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct TradeResponse {
    /// Navigational links related to the trade.
    #[serde(rename = "_links")]
    links: TradeResponseLinks,
    // A unique identifier for this trade.
    id: String,
    /// A pointer to a specific location in a collection of responses, used for pagination control.
    paging_token: String,
    // An ISO 8601 formatted string of when the ledger with this trade was closed.
    ledger_close_time: String,
    // Can be set to `all`, `orderbook`, or `liquidity_pools` to filter only trades executed across a given mechanism.
    trade_type: String,
    // The value for the liquidity pool fee's base points. 
    liquidity_pool_fee_bp: Option<u32>,
    // The base liquidity pool ID, if this trade was executed against a liquidity pool.
    base_liquidity_pool_id: Option<String>,
    // The base offer ID.
    base_offer_id: String,
    // The account ID of the base party for this trade.
    base_account: String,
    // The amount of the base asset that was moved from `base_account` to `counter_account`.
    base_amount: String,
    // The type for the base asset. Either `native`, `credit_alphanum4`, or `credit_alphanum12`.
    base_asset_type: Option<String>,
    // The code for the base asset.
    base_asset_code: Option<String>,
    // The Stellar address of the base asset’s issuer.
    base_asset_issuer: Option<String>,
    // The counter liquidity pool ID, if this trade was executed against a liquidity pool.
    counter_liquidity_pool_id: Option<String>,
    // The counter offer ID.
    counter_offer_id: String,
    // The account ID of the counter party for this trade.
    counter_account: String,
    // The amount of the counter asset that was moved from `counter_account` to `base_account`.
    counter_amount: String,
    // The type for the counter asset. Either `native`, `credit_alphanum4`, or `credit_alphanum12`.
    counter_asset_type: Option<String>,
    // The code for the counter asset.
    counter_asset_code: Option<String>,
    // The Stellar address of the counter asset’s issuer.
    counter_asset_issuer: Option<String>,
    // Indicates with party is the seller.
    base_is_seller: bool,
    // An object of a number numerator and number denominator that represents the original offer price.
    price: Option<Price>,
}

impl Response for TradeResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}