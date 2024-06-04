use crate::models::{Order, Request};
use crate::BuildQueryParametersExt;
use crate::Paginatable;
use stellar_rust_sdk_derive::Pagination;

/// Represents the request to fetch effects for a specific liquidity pool from the Horizon API.

/// `EffectsForLiquidityPoolRequest` is a struct used to construct queries for retrieving information about effects
/// from the Horizon server. It includes parameters that allow for pagination control and sorting
/// of the effect records.

/// # Usage
/// Instances of `EffectsForLiquidityPoolRequest` are created and optionally configured using the builder pattern.
/// Once the desired parameters are set, the request can be passed to the Horizon client to fetch
/// effect data.
///
/// # Fields
/// * `liquidity_pool_id` - The liquidity pool id.
/// * `cursor` - A pointer to a specific location in a collection of responses, derived from the
///   `paging_token` value of a record. Used for pagination control in the API response.
/// * `limit` - Specifies the maximum number of records to be returned in a single response.
///
/// # Example
/// ```rust
/// # use stellar_rs::effects::effects_for_liquidity_pools_request::EffectsForLiquidityPoolRequest;
/// # use stellar_rs::models::*;
/// # use stellar_rust_sdk_derive::Pagination;
/// # use stellar_rs::Paginatable;
///
/// let request = EffectsForLiquidityPoolRequest::new()
///     .set_liquidity_pool_id("01c58ab8fb283c8b083a26bf2fe06b7b6c6304c13f9d29d956cdf15a48bea72d".to_string())
///     .set_cursor(1234).unwrap()
///     .set_limit(20).unwrap()
///     .set_order(Order::Desc);
///
/// // The request can now be used with a Horizon client to fetch effects.
/// ```
///
#[derive(Default, Pagination)]
pub struct EffectsForLiquidityPoolRequest {
    /// The liquidity pool id
    liquidity_pool_id: Option<String>,
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

impl EffectsForLiquidityPoolRequest {
    /// Creates a new `EffectsForLiquidityPoolsRequest` with default parameters.
    pub fn new() -> Self {
        EffectsForLiquidityPoolRequest::default()
    }

    /// Sets the liquidity pool id for the request.
    ///
    /// # Arguments
    /// * `liquidity_pool_id` - A `String` value representing the liquidity pool id.
    ///
    pub fn set_liquidity_pool_id(
        self,
        liquidity_pool_id: String,
    ) -> EffectsForLiquidityPoolRequest {
        EffectsForLiquidityPoolRequest {
            liquidity_pool_id: Some(liquidity_pool_id),
            ..self
        }
    }
}

impl Request for EffectsForLiquidityPoolRequest {
    //TODO research different url buildig methods
    fn get_query_parameters(&self) -> String {
        vec![
            self.liquidity_pool_id
                .as_ref()
                .map(|l| format!("liquidity_pool_id={}", l)),
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
    use crate::{BuildQueryParametersExt};

    #[test]
    fn test_effects_for_liquidity_pools_request() {
        let request = EffectsForLiquidityPoolRequest::new()
            .set_liquidity_pool_id("liquidity_pool_id".to_string())
            .set_cursor(1)
            .unwrap()
            .set_limit(10)
            .unwrap()
            .set_order(Order::Asc)
            .unwrap();

        let url = request.build_url("https://horizon-testnet.stellar.org");
        let query_parameters = vec![
            Some("liquidity_pool_id=liquidity_pool_id".to_string()),
            Some("cursor=1".to_string()),
            Some("limit=10".to_string()),
            Some("order=asc".to_string()),
        ]
        .build_query_parameters();

        assert_eq!(
            url,
            "https://horizon-testnet.stellar.org/effects?liquidity_pool_id=liquidity_pool_id&cursor=1&limit=10&order=asc"
        );
        assert_eq!(
            query_parameters,
            "?liquidity_pool_id=liquidity_pool_id&cursor=1&limit=10&order=asc"
        );
    }
}
