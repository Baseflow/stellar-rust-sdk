use derive_getters::Getters;
use serde::Deserialize;

use crate::{models::Response, Embedded, ResponseLinks};

use super::{LedgerRecord, LedgerRecordLinks};

/// Represents the response to a request for listing all ledgers from the Stellar Horizon API.
///
/// This struct contains the overall structure of the response for querying all ledgers. It includes
/// navigational links and a collection of ledger records, each with comprehensive details about the ledger.
///
#[derive(Debug, Deserialize, Clone, Getters)]
pub struct LedgersResponse {
    /// Navigational links for the current, next, and previous pages of the response.
    #[serde(rename = "_links")]
    links: ResponseLinks,
    /// Contains the actual list of ledger records in the `records` field.
    #[serde(rename = "_embedded")]
    embedded: Embedded<LedgerRecord>,
}

impl Response for LedgersResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}
