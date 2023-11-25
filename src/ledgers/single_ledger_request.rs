use crate::models::*;

pub struct SingleLedgerRequest {
    /// The sequence of the ledger
    sequence: u32,
}

impl Request for SingleLedgerRequest {
    fn new() -> Self {
        Self { sequence: 0 }
    }

    fn get_query_parameters(&self) -> String {
        format!("{}", self.sequence)
    }

    fn validate(&self) -> Result<(), String> {
        if self.sequence < 1 {
            return Err("sequence must be greater than or equal to 1".to_string());
        }

        Ok(())
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}/{}",
            base_url,
            super::LEDGERS_PATH,
            self.get_query_parameters()
        )
    }
}

impl SingleLedgerRequest {
    /// Sets the sequence
    /// # Arguments
    /// * `sequence` - The sequence
    /// # Returns
    /// The request object
    /// [SingleLedgerRequest](struct.SingleLedgerRequest.html)
    pub fn set_sequence(&mut self, sequence: u32) -> &mut Self {
        self.sequence = sequence;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledgers_request() {
        let request = SingleLedgerRequest::new();
        assert_eq!(request.build_url("https://horizon-testnet.stellar.org"), "https://horizon-testnet.stellar.org/ledgers/0");
    }
}
