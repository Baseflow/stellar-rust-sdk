use crate::{
    models::{Order, Request},
    BuildQueryParametersExt,
    Paginatable,
};
use stellar_rust_sdk_derive::Pagination;

/// Represents the request to fetch effects for a specific account from the Horizon API.
///
/// `EffectsForAccountRequest` is a struct used to construct queries for retrieving information about effects
/// from the Horizon server. It includes parameters that allow for pagination control and sorting
/// of the effect records.
///
/// # Usage
/// Instances of `EffectsForAccountRequest` are created and optionally configured using the builder pattern.
/// Once the desired parameters are set, the request can be passed to the Horizon client to fetch
/// effect data.
///
/// # Fields
/// * `account_id` - The account's public id.
/// * `cursor` - A pointer to a specific location in a collection of responses, derived from the
/// * `limit` - Specifies the maximum number of records to be returned in a single response.
/// * `order` - Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
///
/// # Example
/// ```rust
/// # use stellar_rs::effects::effects_for_account_request::EffectsForAccountRequest;
/// # use stellar_rs::models::*;
/// # use stellar_rust_sdk_derive::Pagination;
/// # use crate::stellar_rs::Paginatable;
///
/// let request = EffectsForAccountRequest::new()
///     .set_cursor(1234).unwrap()
///     .set_limit(20).unwrap()
///     .set_order(Order::Desc);
///
/// // The request can now be used with a Horizon client to fetch effects.
/// ```
///

#[derive(Default, Pagination)]
pub struct EffectsForAccountRequest {
    /// The accounts public id
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
}

impl EffectsForAccountRequest {
    /// Creates a new `EffectForAccountRequest` with default parameters.
    pub fn new() -> Self {
        EffectsForAccountRequest::default()
    }

    /// Sets the account id for the request.
    ///
    /// # Arguments
    /// * `account_id` - A `String` value representing the account id.
    ///
    pub fn set_account_id(self, account_id: String) -> EffectsForAccountRequest {
        EffectsForAccountRequest {
            account_id: Some(account_id),
            ..self
        }
    }
}

impl Request for EffectsForAccountRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.account_id.as_ref().map(|a| format!("account={}", a)),
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
    fn test_effects_for_account_request() {
        let request = EffectsForAccountRequest::new();
        assert_eq!(
            request.build_url("https://horizon-testnet.stellar.org"),
            "https://horizon-testnet.stellar.org/effects"
        );
    }

    #[test]
    fn test_effects_for_account_request_with_params() {
        let request = EffectsForAccountRequest::new()
            .set_account_id("GBL3QJ2MB3KJ7YV7YVXJ5ZL5V6Z5ZL5V6Z5ZL5V6Z5ZL5V6Z5ZL5V6Z".to_string())
            .set_cursor(1)
            .unwrap()
            .set_limit(10)
            .unwrap()
            .set_order(Order::Desc)
            .unwrap();
        assert_eq!(
            request.build_url("https://horizon-testnet.stellar.org"),
            "https://horizon-testnet.stellar.org/effects?account=GBL3QJ2MB3KJ7YV7YVXJ5ZL5V6Z5ZL5V6Z5ZL5V6Z5ZL5V6Z5ZL5V6Z&cursor=1&limit=10&order=desc"
        );
    }

    #[test]
    fn test_effects_for_account_request_set_limit() {
        let invalid_limit: u8 = 255;

        let request = EffectsForAccountRequest::new().set_limit(invalid_limit);

        assert!(request.is_err());
    }

    #[test]
    fn test_effects_for_account_request_set_cursor() {
        let invalid_cursor = 0;

        let request = EffectsForAccountRequest::new().set_cursor(invalid_cursor);

        assert!(request.is_err());
    }
}
