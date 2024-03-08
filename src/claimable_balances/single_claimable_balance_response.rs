use derive_getters::Getters;
use serde::Deserialize;

use crate::models::Response;

use super::ClaimableBalanceRecord;

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SingleClaimableBalanceResponse {
    pub record: ClaimableBalanceRecord,
}

impl Response for SingleClaimableBalanceResponse {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(SingleClaimableBalanceResponse { record: response })
    }
}
