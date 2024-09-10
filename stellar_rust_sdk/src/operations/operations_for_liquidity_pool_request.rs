use crate::{
    models::{IncludeFailed, Order, Request},
    BuildQueryParametersExt,
};
use stellar_rust_sdk_derive::pagination;

#[pagination]
#[derive(Default)]
pub struct OperationsForLiquidityPoolRequest {
    /// A unique identifier for the liquidity pool of the operation(s).
    liquidity_pool_id: Option<String>,
    /// Set to true to include failed operations in results. Options include true and false.
    include_failed: Option<IncludeFailed>,
}

impl OperationsForLiquidityPoolRequest {
    pub fn new() -> Self {
        OperationsForLiquidityPoolRequest::default()
    }

    /// Sets whether to include failed operations in the response.
    ///
    /// # Arguments
    /// * `include_failed` - A boolean value that determines whether to include failed operations in the response.
    ///
    pub fn set_include_failed(
        self,
        include_failed: IncludeFailed,
    ) -> OperationsForLiquidityPoolRequest {
        OperationsForLiquidityPoolRequest {
            include_failed: Some(include_failed),
            ..self
        }
    }

    /// Sets the account ID for which to retrieve operations.
    ///
    /// # Arguments
    /// * `account_id` - A `String` representing the account ID.
    ///
    pub fn set_liquidity_pool_id(
        self,
        liquidity_pool_id: impl Into<String>,
    ) -> OperationsForLiquidityPoolRequest {
        OperationsForLiquidityPoolRequest {
            liquidity_pool_id: Some(liquidity_pool_id.into()),
            ..self
        }
    }
}

impl Request for OperationsForLiquidityPoolRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
            self.include_failed
                .as_ref()
                .map(|i| format!("include_failed={}", i)),
        ]
        .build_query_parameters()
    }
    fn build_url(&self, base_url: &str) -> String {
        let binding = "".to_string();
        let liquidity_pool_id = self.liquidity_pool_id.as_ref().unwrap_or(&binding);
        format!(
            "{}/liquidity_pools/{}/{}{}",
            base_url,
            liquidity_pool_id,
            super::OPERATIONS_PATH,
            self.get_query_parameters(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Order;

    #[test]
    fn test_all_operations_request() {
        let request = OperationsForLiquidityPoolRequest::new()
            .set_cursor(1)
            .unwrap()
            .set_limit(10)
            .unwrap()
            .set_order(Order::Desc)
            .unwrap()
            .set_include_failed(IncludeFailed::True);

        assert_eq!(
            request.get_query_parameters(),
            "?cursor=1&limit=10&order=desc&include_failed=true"
        );
    }
}
