use crate::models::{IncludeFailed, Order, Request};
use crate::payments::PAYMENTS_PATH;
use crate::Paginatable;
use stellar_rust_sdk_derive::Pagination;

#[derive(Default, Pagination)]
pub struct PaymentsForLedgerRequest {
    /// The Stellar address of the account for which you want to retrieve payments.
    ledger_sequence: Option<String>,
    /// A pointer to a specific location in a collection of responses, derived from the
    ///  `paging_token` value of a record. Used for pagination control in the API response.
    cursor: Option<u32>,
    /// Specifies the maximum number of records to be returned in a single response.
    limit: Option<u8>,
    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    /// and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    order: Option<Order>,
    /// A boolean value that determines whether failed transactions should be included in the response.
    include_failed: IncludeFailed,
}

impl PaymentsForLedgerRequest {
    /// Creates a new `PaymentsForAccountRequest` with default parameters.
    pub fn new() -> PaymentsForLedgerRequest {
        PaymentsForLedgerRequest {
            ledger_sequence: None,
            cursor: None,
            limit: None,
            order: None,
            include_failed: IncludeFailed::False,
        }
    }

    /// Sets the Stellar address of the account for which you want to retrieve payments.
    ///
    /// # Arguments
    /// * `account_id` - The Stellar address of the account for which you want to retrieve payments.
    ///
    pub fn set_ledger_sequence(mut self, ledger_sequence: String) -> PaymentsForLedgerRequest {
        self.ledger_sequence = Some(ledger_sequence);
        self
    }

    /// Sets whether to include failed operations in the response.
    ///
    /// # Arguments
    /// * `include_failed` - A boolean value that determines whether to include failed operations in the response.
    ///
    pub fn set_include_failed(mut self, include_failed: IncludeFailed) -> PaymentsForLedgerRequest {
        self.include_failed = include_failed;
        self
    }
}

impl Request for PaymentsForLedgerRequest {
    fn get_query_parameters(&self) -> String {
        let mut params = String::new();
        if let Some(cursor) = self.cursor {
            params.push_str(&format!("cursor={}&", cursor));
        }
        if let Some(limit) = self.limit {
            params.push_str(&format!("limit={}&", limit));
        }
        if let Some(order) = &self.order {
            params.push_str(&format!("order={}&", order));
        }
        params.push_str(&format!("include_failed={}", self.include_failed));
        params
    }

    fn build_url(&self, base_url: &str) -> String {
        let binding = "".to_string();
        let ledger_sequence = self.ledger_sequence.as_ref().unwrap_or(&binding);
        format!(
            "{}/ledgers/{}/{}?{}",
            base_url,
            ledger_sequence,
            PAYMENTS_PATH,
            self.get_query_parameters()
        )
    }
}
