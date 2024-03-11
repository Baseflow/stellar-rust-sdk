use crate::models::Request;
#[derive(Default, Clone)]
pub struct LiquidityPoolId(String);
#[derive(Default, Clone)]
pub struct NoLiquidityPoolId;

/// Represents a request to fetch details of a single liquidity pool from the Horizon API.
///
/// `SingleLiquidityPoolRequest` is a struct tailored to querying details of a specific liquidity pool
/// on the Horizon API. This struct is designed to be used in conjunction with the
/// [`HorizonClient::get_single_liquidity_pool`](crate::horizon_client::HorizonClient::get_single_liquidity_pool) method.
///
/// The struct matches the parameters necessary to construct a request for the
/// <a href="https://developers.stellar.org/api/horizon/resources/liquidity-pools/single/">Retrieve A Liquidity Pool endpoint</a>
/// of the Horizon API.
///
/// # Fields
/// Required:
/// * `liquidity_pool_id` - The liquidity pool's ID.
///
/// ## Usage
/// Instances of `SingleLiquidityPoolRequest` are created and configured using setter methods for each
/// parameter.
/// ```
/// # use stellar_rs::liquidity_pools::prelude::SingleLiquidityPoolRequest;
/// # use stellar_rs::models::Request;
/// let request = SingleLiquidityPoolRequest::new()
///     .set_liquidity_pool_id("1".to_string())
///     .unwrap();
/// // Use with HorizonClient::get_single_liquidity_pool
/// ```
///
#[derive(Default)]
pub struct SingleLiquidityPoolRequest<I> {
    /// The liquidity pool's ID.
    pub liquidity_pool_id: I,
}

impl SingleLiquidityPoolRequest<NoLiquidityPoolId> {
    /// Creates a new `SingleLiquidityPoolRequest` with default parameters.
    pub fn new() -> Self {
        SingleLiquidityPoolRequest::default()
    }

    /// Sets the liquidity pool ID for the request.
    ///
    /// # Arguments
    /// * `liquidity_pool_id` - A `String` specifying the liquidity pool's ID.
    ///
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