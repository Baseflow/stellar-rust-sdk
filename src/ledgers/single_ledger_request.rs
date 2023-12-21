use crate::models::*;

/// Represents a ledger sequence number.
#[derive(Default, Clone)]
pub struct Sequence(u32);

/// Represents the absence of a ledger sequence number.
#[derive(Default, Clone)]
pub struct NoSequence;

/// `SingleLedgerRequest` is used to request information for a specific ledger from the Stellar Horizon API.
///
/// This struct allows users to specify the sequence number of a ledger in order to retrieve its details.
/// It is typically used in conjunction with the `HorizonClient` to make API calls to the Horizon server.
///
/// # Fields
/// Required:
/// * `sequence`: The sequence number of the ledger to be retrieved.
///
/// ## Usage
/// ```
/// # use stellar_rs::ledgers::prelude::SingleLedgerRequest;
/// # use stellar_rs::models::Request;
/// let request = SingleLedgerRequest::new()
///     .set_sequence(12345); // Example sequence
///
/// // Use with HorizonClient::get_single_ledger
/// ```
///
#[derive(Default)]
pub struct SingleLedgerRequest<S> {
    /// The sequence number of the ledger to be retrieved.
    sequence: S,
}

impl SingleLedgerRequest<NoSequence> {
    /// Creates a new `SingleLedgerRequest` instance with no specified sequence.
    pub fn new() -> Self {
        SingleLedgerRequest::default()
    }

    /// Sets the ledger sequence number for the request.
    ///
    /// # Arguments
    /// * `sequence` - The ledger sequence number to retrieve.
    ///
    /// # Returns
    /// A `SingleLedgerRequest` with the specified sequence number, or an error if the sequence number is invalid.
    ///
    pub fn set_sequence(self, sequence: u32) -> Result<SingleLedgerRequest<Sequence>, String> {
        if sequence < 1 {
            return Err("sequence must be greater than or equal to 1".to_string());
        }

        Ok(SingleLedgerRequest {
            sequence: Sequence(sequence),
        })
    }
}

impl Request for SingleLedgerRequest<Sequence> {
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

