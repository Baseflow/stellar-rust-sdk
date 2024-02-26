use crate::models::{Order, Request};
use crate::BuildQueryParametersExt;

#[derive(Default)]
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

    /// Sets the cursor for pagination.
    ///
    /// # Arguments
    /// * `cursor` - A `u32` value pointing to a specific location in a collection of responses.
    ///
    pub fn set_cursor(self, cursor: u32) -> Result<EffectsForLiquidityPoolRequest, String> {
        if cursor < 1 {
            return Err("cursor must be greater than or equal to 1".to_string());
        }

        Ok(EffectsForLiquidityPoolRequest {
            cursor: Some(cursor),
            ..self
        })
    }

    /// Sets the maximum number of records to return.
    ///
    /// # Arguments
    /// * `limit` - A `u8` value specifying the maximum number of records. Range: 1 to 200. Defaults to 10.
    ///
    pub fn set_limit(self, limit: u8) -> Result<EffectsForLiquidityPoolRequest, String> {
        if limit < 1 || limit > 200 {
            return Err("limit must be between 1 and 200".to_string());
        }

        Ok(EffectsForLiquidityPoolRequest {
            limit: Some(limit),
            ..self
        })
    }

    /// Sets the order of the returned records.
    ///
    /// # Arguments
    /// * `order` - An [`Order`] enum value specifying the order (ascending or descending).
    ///
    pub fn set_order(self, order: Order) -> EffectsForLiquidityPoolRequest {
        EffectsForLiquidityPoolRequest {
            order: Some(order),
            ..self
        }
    }
}


//TODO research different url buildig methods
impl Request for EffectsForLiquidityPoolRequest {
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
    use crate::BuildQueryParametersExt;

    #[test]
    fn test_effects_for_liquidity_pools_request() {
        let request = EffectsForLiquidityPoolRequest::new()
            .set_liquidity_pool_id("liquidity_pool_id".to_string())
            .set_cursor(1)
            .unwrap()
            .set_limit(10)
            .unwrap()
            .set_order(Order::Asc);

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
        assert_eq!(query_parameters, "?liquidity_pool_id=liquidity_pool_id&cursor=1&limit=10&order=asc");
    }
}