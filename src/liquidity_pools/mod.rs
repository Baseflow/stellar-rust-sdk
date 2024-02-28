pub mod all_liquidity_pools_request;
pub mod all_liquidity_pools_response;
pub mod single_liquidity_pool_response;
pub mod single_liquidity_pool_request;

static LIQUIDITY_POOLS_PATH: &str = "liquidity_pools";

pub mod prelude {
    pub use super::all_liquidity_pools_request::*;
    pub use super::all_liquidity_pools_response::*;
    pub use super::single_liquidity_pool_request::*;
    pub use super::single_liquidity_pool_response::*;
}