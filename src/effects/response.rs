use derive_getters::Getters;
use serde::Deserialize;

use crate::models::prelude::*;

/// Represents the navigational links belonging to an effect from the Stellar Horizon API.
///
/// This struct includes links such as the operation (current effect), succeeds, and precedes,
/// providing quick navigation across operational sequence belonging to the effect.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct EffectLink {
    /// The link to the current operation of the effect.
    operation: Link,
    /// The link to the effect succeeding the current operation of the effect.
    succeeds: Link,
    /// The link to the effect preceding the current operation of the effect.
    precedes: Link,
}

/// Represents a single effect record in the Horizon API response.
///
/// This struct encapsulates detailed information about a single effect, including its ID,
/// account, effect type, timestamps, and other relevant data.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Effect {
    /// Navigational links related to the operation of the effect.
    #[serde(rename = "_links")]
    pub links: EffectLink,
    /// The unique identifier of the account.
    pub id: String,
    /// A token used for paging through results.
    pub paging_token: String,
    /// The ID of the account related to the effect.
    pub account: String,
    /// The type of the effect.
    #[serde(rename = "type")]
    pub effect_type: String,
    /// The integer representation of the effect type.
    pub type_i: u32,
    /// The timestamp when the effect was created.
    pub created_at: String,
    /// The starting balance of the account, applicable for certain effect types.
    pub starting_balance: Option<String>,
    /// The type of the asset involved in the effect, if applicable.
    pub asset_type: Option<String>,
    /// The amount of the asset transacted in the effect, if applicable.
    pub amount: Option<String>,
}

/// Represents the response to a request for listing all effects from the Stellar Horizon API.
///
/// This struct contains the overall structure of the response for querying all effects. It includes
/// navigational links and a collection of effect records, each with comprehensive details about the effect.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct EffectsResponse {
    /// Navigational links for the current, next, and previous pages of the response.
    #[serde(rename = "_links")]
    links: ResponseLinks,
    /// Contains the actual list of effect records in the `records` field.

    #[serde(rename = "_embedded")]
    embedded: Embedded<Effect>,
}

impl Response for EffectsResponse {
    fn from_json(json: String) -> Result<EffectsResponse, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}
