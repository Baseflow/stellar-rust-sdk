use stellar_rust_sdk_derive::pagination;

use crate::{
    models::{Order, Request},
    BuildQueryParametersExt,
};

#[pagination]
#[derive(Default)]
pub struct OperationsForTransactionRequest {
    /// The hash of the transaction. Optional.
    transaction_hash: Option<String>,
}

impl OperationsForTransactionRequest {
    pub fn new() -> Self {
        OperationsForTransactionRequest::default()
    }

    /// Sets the transaction hash for which to retrieve operations.
    ///
    /// # Arguments
    /// * `transaction_hash` - A `String` representing the transaction hash.
    ///
    pub fn set_transaction_hash(self, transaction_hash: impl Into<String>) -> OperationsForTransactionRequest {
        OperationsForTransactionRequest {
            transaction_hash: Some(transaction_hash.into()),
            ..self
        }
    }
}

impl Request for OperationsForTransactionRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        let transaction_hash = &self.transaction_hash.as_ref().unwrap();
        use crate::transactions::TRANSACTIONS_PATH;
        format!(
            "{}/{}/{}/{}{}",
            base_url,
            TRANSACTIONS_PATH,
            transaction_hash,
            super::OPERATIONS_PATH,
            self.get_query_parameters(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Order;

    #[test]
    fn test_operations_for_transaction_request() {
        let request = OperationsForTransactionRequest::new()
            .set_cursor(1)
            .unwrap()
            .set_limit(10)
            .unwrap()
            .set_order(Order::Desc)
            .unwrap();

        assert_eq!(
            request.get_query_parameters(),
            "?cursor=1&limit=10&order=desc"
        );
    }
}
