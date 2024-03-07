use crate::models::Response;

use super::ClaimableBalanceRecord;

impl Response for ClaimableBalanceRecord {
    fn from_json(json: String) -> Result<Self, String> {
        let response = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(response)
    }
}
