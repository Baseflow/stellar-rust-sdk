use crate::models::Request;
#[derive(Default, Clone)]
pub struct LiquidityPoolId(String);
#[derive(Default, Clone)]
pub struct NoLiquidityPoolId;

#[derive(Default)]
pub struct SingleLiquidityPoolRequest<I> {
    pub liquidity_pool_id: I,
}

impl SingleLiquidityPoolRequest<NoLiquidityPoolId> {
    pub fn new() -> Self {
        SingleLiquidityPoolRequest::default()
    }

    pub fn set_liquidity_pool_id(
        self,
        liquidity_pool_id: String,
    ) -> Result<SingleLiquidityPoolRequest<LiquidityPoolId>, String> {

        Ok(SingleLiquidityPoolRequest {
            liquidity_pool_id: LiquidityPoolId(liquidity_pool_id),
        })
    }
}

impl Request for SingleLiquidityPoolRequest<LiquidityPoolId> {
    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        query.push_str(&format!("{}", self.liquidity_pool_id.0));

        query.trim_end_matches('&').to_string()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}/{}",
            base_url,
            super::LIQUIDITY_POOLS_PATH,
            self.get_query_parameters()
        )
    }
}