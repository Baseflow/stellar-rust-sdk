use crate::models::{Order, Request};
use crate::payments::PAYMENTS_PATH;
use crate::BuildQueryParametersExt;
use stellar_rust_sdk_derive::pagination;

#[pagination]
#[derive(Default)]
pub struct PaymentsForTransactionRequest {
    /// The transaction hash of the transaction for which you want to retrieve payments.
    transaction_hash: Option<String>,
}

impl PaymentsForTransactionRequest {
    /// Creates a new `PaymentsForTransactionRequest` with default parameters.
    pub fn new() -> PaymentsForTransactionRequest {
        PaymentsForTransactionRequest {
            transaction_hash: None,
            cursor: None,
            limit: None,
            order: None,
        }
    }

    /// Sets the transaction hash of the transaction for which you want to retrieve payments.
    ///
    /// # Arguments
    /// * `transaction_hash` - The transaction hash of the transaction for which you want to retrieve payments.
    ///
    pub fn set_transaction_hash(
        mut self,
        transaction_hash: String,
    ) -> PaymentsForTransactionRequest {
        self.transaction_hash = Some(transaction_hash);
        self
    }
}

impl Request for PaymentsForTransactionRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
            .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        let binding = "".to_string();
        let transaction_hash = self.transaction_hash.as_ref().unwrap_or(&binding);
        format!(
            "{}/transactions/{}/{}?{}",
            base_url,
            transaction_hash,
            PAYMENTS_PATH,
            self.get_query_parameters()
        )
    }
}
