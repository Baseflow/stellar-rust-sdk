use crate::models::*;

#[derive(Default, Clone)]
pub struct Sequence(u32);
#[derive(Default, Clone)]
pub struct NoSequence;

#[derive(Default)]
pub struct SingleLedgerRequest<S> {
    /// The sequence of the ledger
    sequence: S,
}


impl SingleLedgerRequest<NoSequence> {
    pub fn new() -> Self {
        SingleLedgerRequest::default()
    }
}

impl Request for SingleLedgerRequest<Sequence> {
    fn get_path(&self) -> &str {
        "/ledgers"
    }

    fn get_query_parameters(&self) -> String {
        format!("/{}", self.sequence.0)
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

impl<S> SingleLedgerRequest<S> {
    /// Sets the sequence
    /// # Arguments
    /// * `sequence` - The sequence
    /// # Returns
    /// The request object
    /// [SingleLedgerRequest](struct.SingleLedgerRequest.html)
    pub fn set_sequence(
        self,
        sequence: u32,
    ) -> Result<SingleLedgerRequest<Sequence>, String> {
        if sequence < 1 {
            return Err("sequence must be greater than or equal to 1".to_string());
        }

        Ok(
            SingleLedgerRequest {    
                sequence: Sequence(sequence),
            }
        )
    }
}

impl Request for SingleLedgerRequest<Sequence> {
    fn get_path(&self) -> &str {
        "/ledgers"
    }

    fn get_query_parameters(&self) -> String {
        format!("{}", self.sequence.0)
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}{}/{}",
            base_url,
            self.get_path(),
            self.get_query_parameters()
        )
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledgers_request() {
        let request = SingleLedgerRequest::new()
            .set_sequence(2).unwrap();

        assert_eq!(request.get_path(), "/ledgers");
    }
}
