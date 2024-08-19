use stellar_rust_sdk_derive::Pagination;
use crate::Paginatable;

use crate::{
    models::{Order, Request},
    BuildQueryParametersExt,
};

#[derive(Default, Pagination)]
pub struct OperationsForTransactionRequest {
    /// The hash of the transaction. Optional.
    transaction_hash: Option<String>,
    /// A number that points to a specific location in a collection of responses and is pulled
    /// from the paging_token value of a record.
    cursor: Option<u32>,
    /// The maximum number of records returned. The limit can range from 1 to 200 - an upper limit
    /// that is hardcoded in Horizon for performance reasons. If this argument isn’t designated, it
    /// defaults to 10.
    limit: Option<u8>,
    /// A designation of the [`Order`] in which records should appear. Options include [`Order::Asc`] (ascending)
    /// or [`Order::Desc`] (descending). If this argument isn’t set, it defaults to asc.
    order: Option<Order>,
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
    pub fn set_transaction_hash(self, transaction_hash: String) -> OperationsForTransactionRequest {
        OperationsForTransactionRequest {
            transaction_hash: Some(transaction_hash),
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
