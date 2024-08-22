use crate::{models::*, BuildQueryParametersExt};
use stellar_rust_sdk_derive::pagination;

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
///
/// let request = AllEffectsRequest::new()
///     .set_cursor(1234).unwrap()
///     .set_limit(20).unwrap()
///     .set_order(Order::Desc);
///
/// // The request can now be used with a Horizon client to fetch effects.
/// ```
///
#[pagination]
#[derive(Default)]
pub struct AllEffectsRequest {
    // All fields are injected by the `pagination` macro.
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
