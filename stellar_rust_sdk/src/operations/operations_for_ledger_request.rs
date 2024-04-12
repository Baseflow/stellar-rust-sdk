use stellar_rust_sdk_derive::Pagination;
use crate::Paginatable;

use crate::{
    models::{IncludeFailed, Order, Request},
    BuildQueryParametersExt,
};

#[derive(Default, Pagination)]
pub struct OperationsForLedgerRequest {
    /// The account ID for which to retrieve operations.
    ledger_sequence: Option<String>,

    /// A pointer to a specific location in a collection of responses, derived from the
    ///   `paging_token` value of a record. Used for pagination control in the API response.
    cursor: Option<u32>,

    /// Specifies the maximum number of records to be returned in a single response.
    ///   The range for this parameter is from 1 to 200. The default value is set to 10.
    limit: Option<u8>,

    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    ///   and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    order: Option<Order>,

    /// A boolean value that determines whether to include failed operations in the response.
    include_failed: Option<IncludeFailed>,
}

impl OperationsForLedgerRequest {
    pub fn new() -> Self {
        OperationsForLedgerRequest::default()
    }

    /// Sets whether to include failed operations in the response.
    ///
    /// # Arguments
    /// * `include_failed` - A boolean value that determines whether to include failed operations in the response.
    ///
    pub fn set_include_failed(self, include_failed: IncludeFailed) -> OperationsForLedgerRequest {
        OperationsForLedgerRequest {
            include_failed: Some(include_failed),
            ..self
        }
    }

    /// Sets the account ID for which to retrieve operations.
    ///
    /// # Arguments
    /// * `account_id` - A `String` representing the account ID.
    ///
    pub fn set_account_id(self, ledger_sequence: String) -> OperationsForLedgerRequest {
        OperationsForLedgerRequest {
            ledger_sequence: Some(ledger_sequence),
            ..self
        }
    }
}

impl Request for OperationsForLedgerRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
            self.include_failed
                .as_ref()
                .map(|i| format!("include_failed={}", i)),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        let binding = "".to_string();
        let ledger_sequence = self.ledger_sequence.as_ref().unwrap_or(&binding);

        format!(
            "{}/ledgers/{}/{}{}",
            base_url,
            ledger_sequence,
            super::OPERATIONS_PATH,
            self.get_query_parameters()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operations_for_ledger_request() {
        let request = OperationsForLedgerRequest::new()
            .set_cursor(12345)
            .unwrap()
            .set_limit(200)
            .unwrap()
            .set_order(Order::Desc)
            .unwrap()
            .set_include_failed(IncludeFailed::True)
            .set_account_id("00000000".to_string());

        assert_eq!(
            request.get_query_parameters(),
            "?cursor=12345&limit=200&order=desc&include_failed=true"
        );
    }
}
