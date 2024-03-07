use stellar_xdr::curr::{LedgerHeader, Limits, ReadXdr};

use crate::models::Response;

use super::LedgerRecord;

impl Response for LedgerRecord {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
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
