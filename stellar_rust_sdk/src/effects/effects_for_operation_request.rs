use crate::{
    models::{Order, Request},
    BuildQueryParametersExt,
    Paginatable,
};
use stellar_rust_sdk_derive::Pagination;

/// Represents the request to fetch the effects for a specific operation from the Horizon API.
///
/// `EffectsForOperationRequest` is a struct used to construct queries for retrieving information about effects
/// from the Horizon server. It includes parameters that allow for pagination control and sorting
/// of the effect records.
///
/// # Usage
/// Instances of `EffectsForOperationRequest` are created and optionally configured using the builder pattern.
/// Once the desired parameters are set, the request can be passed to the Horizon client to fetch
/// effect data.
///
/// # Fields
/// * `operation_id` - The operation id to filter effects.
/// * `cursor` - A pointer to a specific location in a collection of responses, derived from the
/// * `limit` - Specifies the maximum number of records to be returned in a single response.
/// * `order` - Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
///
/// # Example
/// ```rust
/// # use stellar_rs::effects::effects_for_operation_request::EffectsForOperationRequest;
/// # use stellar_rs::models::*;
/// # use stellar_rust_sdk_derive::Pagination;
/// # use stellar_rs::Paginatable;
///
/// let request = EffectsForOperationRequest::new()
///     .set_operation_id("123")
///     .set_cursor(1).unwrap()
///     .set_limit(10).unwrap()
///     .set_order(Order::Asc);
///
/// // The request can now be used with a Horizon client to fetch effects.
/// ```
///

#[derive(Default, Pagination)]
pub struct EffectsForOperationRequest {
    /// The operation id to filter effects.
    operation_id: Option<String>,
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

impl EffectsForOperationRequest {
    pub fn new() -> EffectsForOperationRequest {
        EffectsForOperationRequest::default()
    }

    pub fn set_operation_id(self, operation_id: &str) -> EffectsForOperationRequest {
        EffectsForOperationRequest {
            operation_id: Some(operation_id.to_string()),
            ..self
        }
    }
}

impl Request for EffectsForOperationRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.operation_id
                .as_ref()
                .map(|l| format!("operation_id={}", l)),
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
            self.get_query_parameters(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_query_parameters() {
        let request = EffectsForOperationRequest::new()
            .set_operation_id("123")
            .set_cursor(1)
            .unwrap()
            .set_limit(10)
            .unwrap()
            .set_order(Order::Asc)
            .unwrap();

        let query_parameters = request.get_query_parameters();
        assert_eq!(
            query_parameters,
            "?operation_id=123&cursor=1&limit=10&order=asc"
        );
    }

    #[test]
    fn test_build_url() {
        let request = EffectsForOperationRequest::new();
        let base_url = "https://horizon-testnet.stellar.org";
        let url = request.build_url(base_url);
        assert_eq!(url, "https://horizon-testnet.stellar.org/effects");
    }
}