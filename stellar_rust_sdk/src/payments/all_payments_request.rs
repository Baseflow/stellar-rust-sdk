use crate::models::{IncludeFailed, Order, Request};
use crate::{models::*, BuildQueryParametersExt, Paginatable};
use stellar_rust_sdk_derive::Pagination;

#[derive(Default, Pagination)]
pub struct AllPaymentsRequest {
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

impl AllPaymentsRequest {
    /// Creates a new `AllPaymentsRequest` with default parameters.
    pub fn new() -> AllPaymentsRequest {
        AllPaymentsRequest {
            cursor: None,
            limit: None,
            order: None,
        }
    }
}

impl Request for AllPaymentsRequest {
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
        format!("{}/payments?{}", base_url, self.get_query_parameters())
    }
}
