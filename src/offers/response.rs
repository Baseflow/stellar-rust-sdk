use crate::models::prelude::*;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// Represents the asset to buy or to sell.
///
/// This struct details information about the asset to buy or to sell, including its type, 
/// code (optional) and issuer (optional).
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Transaction {
    /// The type of asset (e.g. "credit_alphanum4", "credit_alphanum12").
    asset_type: String,
    /// Optional. The code of the asset.
    asset_code: Option<String>,
    /// Optional. The public key of the issuer.
    asset_issuer: Option<String>,
}

/// Represents the precise buy and sell price of the assets on offer.
///
/// This struct contains a numenator and a denominator, so that the price can be determined
/// in a precise manner.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct PriceR {
    /// The numenator.
    n: u32,
    /// The denominator.
    d: u32,
}

/// Represents the navigational links in a single offer response from the Horizon API.
///
/// This struct includes various hyperlinks such as links to the offer itself
/// and the offer maker.
///
#[derive(Debug, Deserialize, Serialize, Clone, Getters)]
pub struct OfferResponseLinks {
    /// The link to the offer itself.
    #[serde(rename = "self")]
    self_link: Link,
    /// Link to the offer's maker.
    offer_maker: Link,
}

/// Represents the response for a single offer query in the Horizon API.
///
/// This struct defines the overall structure of the response for a single offer query.
/// It includes navigational links, offer identifiers, the seller, the assets to buy and sell,
/// the amount, the price and additional data.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SingleOfferResponse {
    /// Navigational links related to the offer.
    #[serde(rename = "_links")]
    links: OfferResponseLinks,
    /// The unique identifier for the offer.
    id: String,
    /// A token used for paging through results.
    paging_token: String,
    /// The ID of the seller making the offer.
    seller: String,
    /// The asset the offer wants to sell.
    selling: Transaction,
    /// The asset the offer wants to buy.
    buying: Transaction,
    /// The amount of `selling` that the account making this offer is willing to sell.
    amount: String,
    /// A precise representation of the buy and sell price of the assets on offer.
    price_r: PriceR,
    /// A number representing the decimal form of `price_r`.
    price: String,
    /// The sequence number of the last ledger in which the offer was modified.
    last_modified_ledger: u32,
    /// The time at which the offer was last modified.
    last_modified_time: String,
    /// The account ID of the sponsor who is paying the reserves for this offer.
    sponsor: Option<String>,
}

impl Response for SingleOfferResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}