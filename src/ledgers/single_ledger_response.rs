use derive_getters::Getters;
use serde::Deserialize;
use stellar_xdr::curr::{LedgerHeader, Limits, ReadXdr};

use crate::models::Response;

use super::LedgerRecord;

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct SingleLedgerResponse {
    pub record: LedgerRecord,
}

impl Response for SingleLedgerResponse {
    fn from_json<>(json: String) -> Result<Self, String> {
        let ledger_record = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(SingleLedgerResponse { record: ledger_record })
    }
}

impl LedgerRecord {
    /// Decodes the XDR-encoded header of the ledger.
    pub fn decoded_header_xdr(&self) -> Result<LedgerHeader, String> {
        let encoded = self.header_xdr.as_bytes();
        let decoded = LedgerHeader::from_xdr_base64(encoded, Limits::none()).unwrap();
        Ok(decoded)
    }
}
