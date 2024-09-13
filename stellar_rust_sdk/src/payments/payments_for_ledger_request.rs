use crate::models::{IncludeFailed, Order, Request};
use crate::payments::PAYMENTS_PATH;
use crate::BuildQueryParametersExt;
use stellar_rust_sdk_derive::pagination;

#[pagination]
#[derive(Default)]
pub struct PaymentsForLedgerRequest {
    /// The Stellar address of the account for which you want to retrieve payments.
    ledger_sequence: Option<String>,
    /// A boolean value that determines whether failed transactions should be included in the response.
    include_failed: Option<IncludeFailed>,
}

impl PaymentsForLedgerRequest {
    /// Creates a new `PaymentsForAccountRequest` with default parameters.
    pub fn new() -> PaymentsForLedgerRequest {
        PaymentsForLedgerRequest {
            ledger_sequence: None,
            cursor: None,
            limit: None,
            order: None,
            include_failed: Option::from(IncludeFailed::False),
        }
    }

    /// Sets the Stellar address of the account for which you want to retrieve payments.
    ///
    /// # Arguments
    /// * `account_id` - The Stellar address of the account for which you want to retrieve payments.
    ///
    pub fn set_ledger_sequence(
        mut self,
        ledger_sequence: impl Into<String>,
    ) -> PaymentsForLedgerRequest {
        self.ledger_sequence = Some(ledger_sequence.into());
        self
    }

    /// Sets whether to include failed operations in the response.
    ///
    /// # Arguments
    /// * `include_failed` - A boolean value that determines whether to include failed operations in the response.
    ///
    pub fn set_include_failed(mut self, include_failed: IncludeFailed) -> PaymentsForLedgerRequest {
        self.include_failed = Option::from(include_failed);
        self
    }
}

impl Request for PaymentsForLedgerRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.include_failed
                .as_ref()
                .map(|s| format!("include_failed={}", s)),
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
        .build_query_parameters()
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
