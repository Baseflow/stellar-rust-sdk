use crate::{models::*, BuildQueryParametersExt};
use stellar_rust_sdk_derive::Pagination;
use crate::Paginatable;

// TODO: Documentation
#[derive(Default, Clone)]
pub struct TransactionsLedgerId(String);

// TODO: Documentation
#[derive(Default, Clone)]
pub struct NoTransactionsLedgerId;

#[derive(Default, Pagination)]
pub struct TransactionsForLedgerRequest<S> {
    /// The ID of the account for which the transactions are to be retrieved.
    ledger_sequence: S,
    // Indicates whether or not to include failed operations in the response.
    include_failed: Option<bool>,
    /// A pointer to a specific location in a collection of responses, derived from the
    /// `paging_token` value of a record. Used for pagination control in the API response.
    pub cursor: Option<u32>,
    /// Specifies the maximum number of records to be returned in a single response.
    /// The range for this parameter is from 1 to 200. The default value is set to 10.
    pub limit: Option<u8>,
    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    /// and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    pub order: Option<Order>,
}

impl TransactionsForLedgerRequest<NoTransactionsLedgerId> {
    /// Creates a new `TransactionsForLedgerRequest` with default parameters.
    pub fn new() -> Self {
        TransactionsForLedgerRequest::default()
    }

    /// Sets the account ID for the request.
    ///
    /// # Arguments
    /// * `account_id` - The account ID for which the transactions are to be retrieved.
    ///
    /// # Returns
    /// A `TransactionsForAccountRequest` with the specified account ID, or an error if the account ID is invalid.
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
        // TODO: Documentation
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