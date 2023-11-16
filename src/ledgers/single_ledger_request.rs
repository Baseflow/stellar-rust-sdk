use crate::models::*;

pub struct SingleLedgerRequest {
    /// The sequence of the ledger
    sequence: u32,
}

// region: --- States
#[derive(Default, Clone)]
pub struct Sequence(u32);
#[derive(Default, Clone)]
pub struct NoSequence;
// endregion: --- States

#[derive(Default, Clone)]
pub struct SingleLedgerRequestBuilder<S> {
    sequence: S,
}

impl SingleLedgerRequestBuilder<NoSequence> {
    pub fn new() -> Self {
        SingleLedgerRequestBuilder::default()
    }
}

impl<S> SingleLedgerRequestBuilder<S> {
    pub fn sequence(
        self,
        sequence: impl Into<u32>,
    ) -> SingleLedgerRequestBuilder<Sequence> {

        SingleLedgerRequestBuilder {    
            sequence: Sequence(sequence.into()) 
        }
    }
}

impl SingleLedgerRequestBuilder<Sequence> {
    pub fn build(self) -> Result<SingleLedgerRequest, String> {
        Ok(SingleLedgerRequest { 
            sequence: self.sequence.0
        })
    }
}


impl Request for SingleLedgerRequest {
    fn new() -> Self {
        Self { sequence: 0 }
    }

    fn get_query_parameters(&self) -> String {
        format!("/{}", self.sequence)
    }

    fn validate(&self) -> Result<(), String> {
        if self.sequence < 1 {
            return Err("sequence must be greater than or equal to 1".to_string());
        }

        Ok(())
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
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
