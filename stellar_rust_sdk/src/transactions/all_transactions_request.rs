use crate::{models::*, BuildQueryParametersExt};
use stellar_rust_sdk_derive::Pagination;
use crate::Paginatable;

// TODO: Documentation
#[derive(Default, Pagination)]
pub struct AllTransactionsRequest {
    // Indicates whether or not to include failed operations in the response.
    include_failed: Option<bool>,
    /// A pointer to a specific location in a collection of responses, derived from the
    /// `paging_token` value of a record. Used for pagination control in the API response.
    cursor: Option<u32>,
    /// Specifies the maximum number of records to be returned in a single response.
    /// The range for this parameter is from 1 to 200. The default value is set to 10.
    limit: Option<u8>,
    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    /// and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    order: Option<Order>,
}

impl Request for AllTransactionsRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.include_failed.as_ref().map(|i| format!("include_failed={}", i)),
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
        .build_query_parameters()
    }

    // TODO: Documentation
    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::TRANSACTIONS_PATH,
            self.get_query_parameters()
        )
    }
}

impl AllTransactionsRequest {
    /// Creates a new `AllTransactionsRequest` with default parameters.
    pub fn new() -> Self {
        AllTransactionsRequest::default()
    }

    // TODO: Documentation
    pub fn set_include_failed(self, include_failed: bool) -> Result<AllTransactionsRequest, String> {
        Ok(AllTransactionsRequest {
            include_failed: Some(include_failed),
            ..self
        })
    }
}