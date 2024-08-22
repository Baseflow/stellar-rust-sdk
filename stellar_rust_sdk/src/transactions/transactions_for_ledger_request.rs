use crate::{models::*, BuildQueryParametersExt};
use stellar_rust_sdk_derive::pagination;

/// Represents the ID of a ledger for which the transactions are to be retrieved.
#[derive(Default, Clone)]
pub struct TransactionsLedgerId(String);

/// Represents the absence of an ID of a ledger for which the transactions are to be retrieved.
#[derive(Default, Clone)]
pub struct NoTransactionsLedgerId;

#[pagination]
#[derive(Default)]
pub struct TransactionsForLedgerRequest<S> {
    /// The ID of the ledger for which the transactions are to be retrieved.
    ledger_sequence: S,
    // Indicates whether or not to include failed operations in the response.
    include_failed: Option<bool>,
}

impl TransactionsForLedgerRequest<NoTransactionsLedgerId> {
    /// Creates a new `TransactionsForLedgerRequest` with default parameters.
    pub fn new() -> Self {
        TransactionsForLedgerRequest::default()
    }

    /// Sets the ledger ID for the request.
    ///
    /// # Arguments
    /// * `ledger_id` - The ledger ID for which the transactions are to be retrieved.
    ///
    /// # Returns
    /// A `TransactionsForLedgerRequest` with the specified ledger ID.
    ///
    pub fn set_ledger_sequence(
        self,
        ledger_sequence: String,
    ) -> Result<TransactionsForLedgerRequest<TransactionsLedgerId>, String> {
        Ok(TransactionsForLedgerRequest {
            ledger_sequence: TransactionsLedgerId(ledger_sequence),
            include_failed: self.include_failed,
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        })
    }
}

impl TransactionsForLedgerRequest<TransactionsLedgerId> {
    /// Sets the `include_failed` field for the request. Can only be set on a request that
    /// has a set ledger id.
    ///
    /// # Arguments
    /// * `include_failed` - A `bool` to indicate whether or not to include failed operations.
    ///
    /// # Returns
    /// A `TransactionsForLedgerRequest` with the updated `include_failed` field.
    ///
    pub fn set_include_failed(
        self,
        include_failed: bool,
    ) -> Result<TransactionsForLedgerRequest<TransactionsLedgerId>, String> {
        Ok(TransactionsForLedgerRequest {
            ledger_sequence: self.ledger_sequence,
            include_failed: Some(include_failed),
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        })
    }
}

impl Request for TransactionsForLedgerRequest<TransactionsLedgerId> {
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
        // This URL comprises paths and query parameters.
        // Additionally, this request uses the API endpoint for `ledgers`.
        let ledger_sequence = &self.ledger_sequence.0;
        use crate::ledgers::LEDGERS_PATH;
        format!(
            "{}/{}/{}/{}{}",
            base_url,
            LEDGERS_PATH,
            ledger_sequence,
            super::TRANSACTIONS_PATH,
            self.get_query_parameters(),
        )
    }
}
