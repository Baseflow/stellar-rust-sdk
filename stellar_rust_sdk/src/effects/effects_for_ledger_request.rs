use crate::{
    models::{Order, Request},
    BuildQueryParametersExt,
    Paginatable,
};
use stellar_rust_sdk_derive::Pagination;

/// Represents a request to fetch effects associated with a specific ledger from the Stellar Horizon API.
///
/// `EffectsForLedgerRequest` is a struct designed to facilitate the retrieval of effects for a given ledger sequence.
/// It supports pagination, custom limits, and sorting order through its fields, allowing for flexible and efficient
/// data access patterns.
///
/// # Example
/// ```rust
/// # use stellar_rs::effects::effects_for_ledger_request::EffectsForLedgerRequest;
/// # use stellar_rs::models::Order;
/// # use stellar_rust_sdk_derive::Pagination;
/// # use stellar_rs::Paginatable;
///
/// let mut request = EffectsForLedgerRequest::new()
///     .set_sequence(&1000)
///     .set_limit(2);
///
/// // The request is now ready to be used with a Horizon client to fetch effects for the specified ledger.
/// ```
///
#[derive(Default, Pagination)]
pub struct EffectsForLedgerRequest {
    /// The ledger's sequence number for which effects are to be retrieved.
    sequence: Option<u32>,
    /// A pointer to a specific location in a collection of responses, derived from the
    ///   `paging_token` value of a record. Used for pagination control in the API response.
    cursor: Option<u32>,
    /// Specifies the maximum number of records to be returned in a single response.
    ///   The range for this parameter is from 1 to 200. The default value is set to 10.
    limit: Option<u8>,
    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    ///   and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    order: Option<Order>,
}

impl EffectsForLedgerRequest {
    /// Creates a new `LedgersRequest` with default parameters.
    pub fn new() -> Self {
        EffectsForLedgerRequest::default()
    }

    /// Sets the ledger sequence for the request.
    ///
    /// # Arguments
    /// * `sequence` - A `String` value representing the ledger sequence.
    ///
    pub fn set_sequence(self, sequence: &u32) -> EffectsForLedgerRequest {
        EffectsForLedgerRequest {
            sequence: Some(*sequence),
            ..self
        }
    }
}

impl Request for EffectsForLedgerRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        // Extract the sequence as a string if set
        let seq = self
            .sequence
            .as_ref()
            .map_or(String::new(), |s| s.to_string());

        format!(
            "{}/ledgers/{}/{}{}",
            base_url,
            seq,
            super::EFFECTS_PATH,
            self.get_query_parameters()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effects_for_ledger_request_build_url() {
        let sequence: u32 = 125;

        let request = EffectsForLedgerRequest::new().set_sequence(&sequence);

        let url = request.build_url("https://horizon-testnet.stellar.org");

        assert_eq!(
            url,
            format!(
                "https://horizon-testnet.stellar.org/ledgers/{}/{}",
                sequence,
                crate::effects::EFFECTS_PATH
            )
        );
    }

    #[test]
    fn test_effects_for_ledger_request_set_limit() {
        let invalid_limit: u8 = 255;

        let request = EffectsForLedgerRequest::new().set_limit(invalid_limit);

        assert!(request.is_err());
    }

    #[test]
    fn test_effects_for_ledger_request_set_cursor() {
        let invalid_cursor = 0;

        let request = EffectsForLedgerRequest::new().set_cursor(invalid_cursor);

        assert!(request.is_err());
    }
}
