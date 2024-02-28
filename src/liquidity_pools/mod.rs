pub mod all_liquidity_pools_request;
pub mod all_liquidity_pools_response;

static LIQUIDITY_POOLS_PATH: &str = "liquidity_pools";

pub mod prelude {
    pub use super::all_liquidity_pools_request::*;
    pub use super::all_liquidity_pools_response::*;
}