use crate::models::*;
use stellar_rust_sdk_derive::Pagination;
use crate::Paginatable;

// TODO: Documentation
#[derive(Default, Clone)]
pub struct TradeLiquidityPoolId(String);

// TODO: Documentation
#[derive(Default, Clone)]
pub struct NoTradeLiquidityPoolId;
#[derive(Default)]

#[derive(Pagination)]
pub struct TradesForLiquidityPoolRequest<I> {
    // TODO: Documentation
    liquidity_pool_id: I,
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

impl TradesForLiquidityPoolRequest<TradeLiquidityPoolId> {
    // TODO: Documentation
    pub fn new() -> Self {
        TradesForLiquidityPoolRequest::default()
    }

    pub fn set_liquidity_pool_id(
        self,
        liquidity_pool_id: String,
    ) -> Result<TradesForLiquidityPoolRequest<TradeLiquidityPoolId>, String> {
        if let Err(e) = is_public_key(&liquidity_pool_id) {
            return Err(e.to_string());
        }

        Ok(TradesForLiquidityPoolRequest {
            liquidity_pool_id: TradeLiquidityPoolId(liquidity_pool_id),
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        })
    }
}

impl Request for TradesForLiquidityPoolRequest<TradeLiquidityPoolId> {
    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        query.push_str(&format!("{}", self.liquidity_pool_id.0));

        query.trim_end_matches('&').to_string()
    }

    fn build_url(&self, base_url: &str) -> String {
        // This URL is not built with query paramaters, but with the liquidity pool's ID as addition to the path.
        // Therefore there is no `?` but a `/` in the formatted string.
        // Additionally, this request uses the API endpoint for `liquidity_pools`.
        use crate::liquidity_pools::LIQUIDITY_POOLS_PATH;
        format!(
            "{}/{}/{}/{}",
            base_url,
            LIQUIDITY_POOLS_PATH,
            self.get_query_parameters(),
            super::TRADES_PATH
        )
    }
}