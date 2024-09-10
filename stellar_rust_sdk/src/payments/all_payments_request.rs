use crate::models::{Order, Request};
use crate::BuildQueryParametersExt;
use stellar_rust_sdk_derive::pagination;

#[pagination]
#[derive(Default)]
pub struct AllPaymentsRequest {
    // All fields are injected by the `pagination` macro.
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
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!("{}/payments?{}", base_url, self.get_query_parameters())
    }
}
