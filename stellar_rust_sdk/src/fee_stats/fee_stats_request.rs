use crate::models::Request;

/// Represents a request to fetch fee stats from the Stellar Horizon API.
///
/// `FeeStatsRequest` is a struct used to construct queries for retrieving information about fee stats
/// from the Horizon server. This request does not include any parameters.
///
/// # Usage
/// Instances of `FeeStatsRequest` are created and optionally configured using the builder pattern.
/// Once the desired parameters are set, the request can be passed to the Horizon client to fetch
/// fee stats.
///
/// # Example
/// ```rust
/// use stellar_rs::fee_stats::fee_stats_request::FeeStatsRequest;
/// use stellar_rs::models::*;
///
/// let request = FeeStatsRequest::new();
///
/// // The request can now be used with a Horizon client to fetch fee stats.
/// ```
///
#[derive(Default)]
pub struct FeeStatsRequest {}

impl FeeStatsRequest {
    /// Creates a new `FeeStatsRequest` with default parameters.
    pub fn new() -> FeeStatsRequest {
        FeeStatsRequest::default()
    }
}

impl Request for FeeStatsRequest {
    fn get_query_parameters(&self) -> String {
        "".to_string()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!("{}/{}", base_url, super::FEE_STATS_PATH)
    }
}