use crate::models::{Order, Request};
use crate::payments::PAYMENTS_PATH;
use crate::Paginatable;
use stellar_rust_sdk_derive::Pagination;

#[derive(Default, Pagination)]
pub struct PaymentsForTransactionRequest {
    /// The transaction hash of the transaction for which you want to retrieve payments.
    transaction_hash: Option<String>,
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
        params
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
