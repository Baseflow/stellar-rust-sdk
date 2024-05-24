use crate::models::{IncludeFailed, Order, Request};
use crate::Paginatable;
use stellar_rust_sdk_derive::Pagination;

#[derive(Default, Pagination)]
pub struct AllOperationsRequest {
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

impl AllOperationsRequest {
    pub fn new() -> Self {
        AllOperationsRequest::default()
    }

    /// Sets whether to include failed operations in the response.
    ///
    /// # Arguments
    /// * `include_failed` - A boolean value that determines whether to include failed operations in the response.
    ///
    pub fn set_include_failed(self, include_failed: IncludeFailed) -> AllOperationsRequest {
        AllOperationsRequest {
            include_failed: Some(include_failed),
            ..self
        }
    }
}

impl Request for AllOperationsRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
            self.include_failed
                .as_ref()
                .map(|f| format!("include_failed={}", f)),
        ]
        .iter()
        .flatten()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("&")
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}?{}",
            base_url,
            super::OPERATIONS_PATH,
            self.get_query_parameters()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Order;

    #[test]
    fn test_all_operations_request() {
        let request = AllOperationsRequest::new()
            .set_cursor(1)
            .unwrap()
            .set_limit(10)
            .unwrap()
            .set_order(Order::Desc)
            .unwrap()
            .set_include_failed(IncludeFailed::True);

        assert_eq!(
            request.get_query_parameters(),
            "cursor=1&limit=10&order=desc&include_failed=true"
        );
    }
}
