use crate::{models::*, BuildQueryParametersExt, Paginatable};
use stellar_rust_sdk_derive::Pagination;

/// Represents a request to fetch effect data from the Stellar Horizon API.
///
/// `AllEffectsRequest` is a struct used to construct queries for retrieving information about effects
/// from the Horizon server. It includes parameters that allow for pagination control and sorting
/// of the effect records.
///
/// # Usage
/// Instances of `AllEffectsRequest` are created and optionally configured using the builder pattern.
/// Once the desired parameters are set, the request can be passed to the Horizon client to fetch
/// effect data.
///
/// # Example
/// ```rust
/// # use stellar_rs::effects::all_effects_request::AllEffectsRequest;
/// # use stellar_rs::models::*;
/// # use stellar_rust_sdk_derive::Pagination;
/// # use crate::stellar_rs::Paginatable;
///
/// let request = AllEffectsRequest::new()
///     .set_cursor(1234).unwrap()
///     .set_limit(20).unwrap()
///     .set_order(Order::Desc);
///
/// // The request can now be used with a Horizon client to fetch effects.
/// ```
///
#[derive(Default, Pagination)]
pub struct AllEffectsRequest {
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

impl AllEffectsRequest {
    /// Creates a new `AllEffectsRequest` with default parameters.
    pub fn new() -> Self {
        AllEffectsRequest::default()
    }
}

impl Request for AllEffectsRequest {
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
            super::EFFECTS_PATH,
            self.get_query_parameters()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_effects_request_set_limit() {
        let invalid_limit: u8 = 255;

        let request = AllEffectsRequest::new().set_limit(invalid_limit);

        assert!(request.is_err());
    }

    #[test]
    fn test_all_effects_request_set_cursor() {
        let invalid_cursor = 0;

        let request = AllEffectsRequest::new().set_cursor(invalid_cursor);

        assert!(request.is_err());
    }
}