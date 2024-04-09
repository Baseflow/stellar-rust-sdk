use crate::{
    models::{IncludeFailed, Order, Request},
    BuildQueryParametersExt,
};

use super::operations_for_account_request::OperationsForAccountRequest;

#[derive(Default)]
pub struct OperationsForLiquidityPoolRequest {
    /// A unique identifier for the liquidity pool of the operation(s).
    liquidity_pool_id: Option<String>,
    /// A number that points to a specific location in a collection of responses and is pulled
    /// from the paging_token value of a record.
    cursor: Option<u32>,
    /// The maximum number of records returned. The limit can range from 1 to 200 - an upper limit
    /// that is hardcoded in Horizon for performance reasons. If this argument isn’t designated, it
    /// defaults to 10.
    limit: Option<u8>,
    /// A designation of the [`Order`] in which records should appear. Options include [`Order::Asc`] (ascending)
    /// or [`Order::Desc`] (descending). If this argument isn’t set, it defaults to asc.
    order: Option<Order>,
    /// Set to true to include failed operations in results. Options include true and false.
    include_failed: Option<IncludeFailed>,
}

impl OperationsForLiquidityPoolRequest {
    pub fn new() -> Self {
        OperationsForLiquidityPoolRequest::default()
    }

    /// Sets the cursor for pagination.
    ///
    /// # Arguments
    /// * `cursor` - A `u32` value pointing to a specific location in a collection of responses.
    ///
    pub fn set_cursor(self, cursor: u32) -> Result<OperationsForLiquidityPoolRequest, String> {
        if cursor < 1 {
            return Err("cursor must be greater than or equal to 1".to_string());
        }

        Ok(OperationsForLiquidityPoolRequest {
            cursor: Some(cursor),
            ..self
        })
    }

    /// Sets the maximum number of records to return.
    ///
    /// # Arguments
    /// * `limit` - A `u8` value specifying the maximum number of records. Range: 1 to 200. Defaults to 10.
    ///
    pub fn set_limit(self, limit: u8) -> Result<OperationsForLiquidityPoolRequest, String> {
        if limit < 1 || limit > 200 {
            return Err("limit must be between 1 and 200".to_string());
        }

        Ok(OperationsForLiquidityPoolRequest {
            limit: Some(limit),
            ..self
        })
    }

    /// Sets the order of the returned records.
    ///
    /// # Arguments
    /// * `order` - An [`Order`] enum value specifying the order (ascending or descending).
    ///
    pub fn set_order(self, order: Order) -> OperationsForLiquidityPoolRequest {
        OperationsForLiquidityPoolRequest {
            order: Some(order),
            ..self
        }
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
    pub fn set_liquidity_pool_id(self, liquidity_pool_id: String) -> OperationsForLiquidityPoolRequest {
        OperationsForLiquidityPoolRequest {
            liquidity_pool_id: Some(liquidity_pool_id),
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
            .set_include_failed(IncludeFailed::True);

        assert_eq!(
            request.get_query_parameters(),
            "?cursor=1&limit=10&order=desc&include_failed=true"
        );
    }
}
