use derive_getters::Getters;
use serde::Deserialize;

use crate::models::Response;

/// Represents the navigational links in a effect response from the Stellar Horizon API.
///
/// This struct includes links such as the self-link (current page), next, and previous, 
/// providing quick navigation across different pages of the effect response.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct EffectsResponseLink {
    /// The link to the current page of the effect response.
    #[serde(rename = "self")]
    self_link: SelfLink,
    /// Optional link to the next page of effect records.
    next: Option<SelfLink>,
    /// Optional link to the previous page of effect records.
    prev: Option<SelfLink>,
}

/// Represents a self-link in the effect response.
///
/// This struct defines the structure of the self-link (`href`) found in the effects response
/// from the Horizon API. It contains the URL to the current resource or query.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SelfLink {
    /// A `String` representing the hyperlink reference to the current resource or query.
    href: String,
    /// Optionally indicates if the link is templated
    templated: Option<bool>
}

/// Represents the navigational links belonging to an effect from the Stellar Horizon API.
///
/// This struct includes links such as the operation (current effect), succeeds, and precedes, 
/// providing quick navigation across operational sequence belonging to the effect.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct EffectsResponseRecordLink {
    /// The link to the current operation of the effect.
    operation: SelfLink,
    /// The link to the effect succeeding the current operation of the effect.
    succeeds: SelfLink,
    /// The link to the effect preceding the current operation of the effect.
    precedes: SelfLink,
}

/// Represents a single effect record in the Horizon API response.
///
/// This struct encapsulates detailed information about a single effect, including its ID,
/// account, effect type, timestamps, and other relevant data.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Record {
    /// Navigational links related to the operation of the effect.
    pub _links: EffectsResponseRecordLink,
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

/// Contains the embedded effect records in the all effects response.
///
/// This struct encapsulates a collection of effect records (`Records`) returned in the response.
/// It provides a way to access each individual effect record and its detailed information.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct Embedded {
    /// A vector of individual effect records.
    records: Vec<Record>,
}


/// Represents the response to a request for listing all effects from the Stellar Horizon API.
///
/// This struct contains the overall structure of the response for querying all effects. It includes
/// navigational links and a collection of effect records, each with comprehensive details about the effect.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct EffectsResponse {
    /// Navigational links for the current, next, and previous pages of the response.
    _links: EffectsResponseLink,
    /// Contains the actual list of effect records in the `records` field.
    _embedded: Embedded,
}

impl Response for EffectsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}
