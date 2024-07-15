use stellar_rust_sdk_derive::Pagination;
use crate::models::{IncludeFailed, Order, Request};

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
    /// A boolean value that determines whether failed transactions should be included in the response.
    include_failed: IncludeFailed,
}

impl AllPaymentsRequest {
    /// Creates a new `AllPaymentsRequest` with default parameters.
    pub fn new() -> AllPaymentsRequest {
        AllPaymentsRequest {
            cursor: None,
            limit: None,
            order: None,
            include_failed: IncludeFailed::False,
        }
    }

    /// Sets whether failed transactions should be included in the response.
    ///
    /// # Arguments
    /// * `include_failed` - An [`IncludeFailed`] value representing whether failed transactions should be included in the response.
    ///
    pub fn include_failed(mut self, include_failed: IncludeFailed) -> AllPaymentsRequest {
        self.include_failed = include_failed;
        self
    }
}

impl Request for AllPaymentsRequest {
    fn get_query_parameters(&self) -> String {
        
    }

    fn build_url(&self, base_url: &str) -> String {
    }
}