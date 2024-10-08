use crate::{models::*, BuildQueryParametersExt};
use stellar_rust_sdk_derive::pagination;

/// Represents a request to fetch ledger data from the Stellar Horizon API.
///
/// `LedgersRequest` is a struct used to construct queries for retrieving information about ledgers
/// from the Horizon server. It includes parameters that allow for pagination control and sorting
/// of the ledger records.
///
/// # Usage
/// Instances of `LedgersRequest` are created and optionally configured using the builder pattern.
/// Once the desired parameters are set, the request can be passed to the Horizon client to fetch
/// ledger data.
///
/// # Example
/// ```rust
/// # use stellar_rs::ledgers::ledgers_request::LedgersRequest;
/// # use stellar_rs::models::*;
///
/// let request = LedgersRequest::new()
///     .set_cursor(1234).unwrap()
///     .set_limit(20).unwrap()
///     .set_order(Order::Desc).unwrap();
///
/// // The request can now be used with a Horizon client to fetch ledgers.
/// ```
///
#[pagination]
#[derive(Default)]
pub struct LedgersRequest {
    // All fields are injected by the `pagination` macro.
}

impl LedgersRequest {
    /// Creates a new `LedgersRequest` with default parameters.
    pub fn new() -> Self {
        LedgersRequest::default()
    }
}

impl Request for LedgersRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::LEDGERS_PATH,
            self.get_query_parameters()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledgers_request() {
        let request = LedgersRequest::new();
        assert_eq!(
            request.build_url("https://horizon-testnet.stellar.org"),
            "https://horizon-testnet.stellar.org/ledgers"
        );
    }
}
