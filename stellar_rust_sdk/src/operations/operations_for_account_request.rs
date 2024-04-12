use crate::{
    models::{IncludeFailed, Order, Request},
    BuildQueryParametersExt,
};

use stellar_rust_sdk_derive::Pagination;
use crate::Paginatable;

#[derive(Default, Pagination)]
pub struct OperationsForAccountRequest {
    /// The account ID for which to retrieve operations.
    account_id: Option<String>,

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

// 
impl OperationsForAccountRequest {
    pub fn new() -> Self {
        OperationsForAccountRequest::default()
    }

    /// Sets whether to include failed operations in the response.
    ///
    /// # Arguments
    /// * `include_failed` - A boolean value that determines whether to include failed operations in the response.
    ///
    pub fn set_include_failed(self, include_failed: IncludeFailed) -> OperationsForAccountRequest {
        OperationsForAccountRequest {
            include_failed: Some(include_failed),
            ..self
        }
    }

    /// Sets the account ID for which to retrieve operations.
    ///
    /// # Arguments
    /// * `account_id` - A `String` representing the account ID.
    ///
    pub fn set_account_id(self, account_id: String) -> OperationsForAccountRequest {
        OperationsForAccountRequest {
            account_id: Some(account_id),
            ..self
        }
    }
}

impl Request for OperationsForAccountRequest {
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
        let account_id = self.account_id.as_ref().unwrap_or(&binding);
        format!(
            "{}/accounts/{}/{}?{}",
            base_url,
            account_id,
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
    fn test_all_operations_request() {
        let request = OperationsForAccountRequest::new()
            .set_limit(10)
            .unwrap()
            .set_cursor(1)
            .unwrap()
            .set_order(Order::Desc)
            .unwrap()
            .set_include_failed(IncludeFailed::True);

        assert_eq!(
            request.get_query_parameters(),
            "?cursor=1&limit=10&order=desc&include_failed=true"
        );
    }
}
