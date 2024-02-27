pub mod fee_stats_request;
pub mod fee_stats_response;

static FEE_STATS_PATH: &str = "fee_stats";

pub mod prelude {
    pub use super::fee_stats_request::*;
    pub use super::fee_stats_response::*;
}