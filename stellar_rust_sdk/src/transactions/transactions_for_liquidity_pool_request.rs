use crate::{models::*, BuildQueryParametersExt};
use stellar_rust_sdk_derive::Pagination;
use crate::Paginatable;

// TODO: Documentation
#[derive(Default, Clone)]
pub struct TransactionsLiquidityPoolId(String);

// TODO: Documentation
#[derive(Default, Clone)]
pub struct NoTransactionsLiquidityPoolId;

#[derive(Default, Pagination)]
pub struct TransactionsForLiquidityPoolRequest<I> {
    /// The ID of the liquidity pool for which the transactions are to be retrieved.
    liquidity_pool_id: I,
    // Indicates whether or not to include failed operations in the response.
    include_failed: Option<bool>,
    /// A pointer to a specific location in a collection of responses, derived from the
    /// `paging_token` value of a record. Used for pagination control in the API response.
    pub cursor: Option<u32>,
    /// Specifies the maximum number of records to be returned in a single response.
    /// The range for this parameter is from 1 to 200. The default value is set to 10.
    pub limit: Option<u8>,
    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    /// and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    pub order: Option<Order>,
}

impl TransactionsForLiquidityPoolRequest<NoTransactionsLiquidityPoolId> {
    /// Creates a new `TransactionsForLiquidityPoolRequest` with default parameters.
    pub fn new() -> Self {
        TransactionsForLiquidityPoolRequest::default()
    }

    /// Sets the liquidity pool ID for the request.
    ///
    /// # Arguments
    /// * `liquidity_pool_id` - The liquidity pool ID for which the transactions are to be retrieved.
    ///
    /// # Returns
    /// A `TransactionsForLiquidityPoolRequest` with the specified liquidity pool ID, or an error if the liquidity pool ID is invalid.
    ///
    pub fn set_liquidity_pool_id(
        self,
        liquidity_pool_id: String,
    ) -> Result<TransactionsForLiquidityPoolRequest<TransactionsLiquidityPoolId>, String> {
        Ok(TransactionsForLiquidityPoolRequest {
            liquidity_pool_id: TransactionsLiquidityPoolId(liquidity_pool_id),
            include_failed: self.include_failed,
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        })
    }
}

impl TransactionsForLiquidityPoolRequest<TransactionsLiquidityPoolId> {
    pub fn set_include_failed(
        self,
        include_failed: bool,
    ) -> Result<TransactionsForLiquidityPoolRequest<TransactionsLiquidityPoolId>, String> {
        Ok(TransactionsForLiquidityPoolRequest {
            liquidity_pool_id: self.liquidity_pool_id,
            include_failed: Some(include_failed),
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        })
    }
}

impl Request for TransactionsForLiquidityPoolRequest<TransactionsLiquidityPoolId> {
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
        // TODO: Documentation
        let liquidity_pool_id = &self.liquidity_pool_id.0;
        use crate::liquidity_pools::LIQUIDITY_POOLS_PATH;
        format!(
            "{}/{}/{}/{}{}",
            base_url,
            LIQUIDITY_POOLS_PATH,
            liquidity_pool_id,
            super::TRANSACTIONS_PATH,
            self.get_query_parameters(),
        )
    }
}