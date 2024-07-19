use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::models::Response;

/// Represents the response from the Stellar Horizon API when requesting fee stats.
///
/// This struct encapsulates detailed information about the fee stats, including the last ledger,
/// last ledger base fee, ledger capacity usage, and the fee charged and max fee for transactions.
///
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct FeeStatsResponse {
    /// The last ledger number.
    #[serde(rename = "last_ledger")]
    pub last_ledger: String,
    /// The last ledger base fee.
    #[serde(rename = "last_ledger_base_fee")]
    pub last_ledger_base_fee: String,
    /// The ledger capacity usage.
    #[serde(rename = "ledger_capacity_usage")]
    pub ledger_capacity_usage: String,
    /// The fee charged for transactions.
    #[serde(rename = "fee_charged")]
    pub fee_charged: Fee,
    /// The maximum fee for transactions.
    #[serde(rename = "max_fee")]
    pub max_fee: Fee,
}

/// Represents the fee charged and max fee for transactions.
///
/// This struct encapsulates detailed information about the fee charged and max fee for transactions,
/// including the max, min, mode, and percentile values.
///
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
    /// The maximum fee for transactions.
    pub max: String,
    /// The minimum fee for transactions.
    pub min: String,
    /// The mode fee for transactions.
    pub mode: String,
    /// The 10th percentile fee for transactions.
    pub p10: String,
    /// The 20th percentile fee for transactions.
    pub p20: String,
    /// The 30th percentile fee for transactions.
    pub p30: String,
    /// The 40th percentile fee for transactions.
    pub p40: String,
    /// The 50th percentile fee for transactions.
    pub p50: String,
    /// The 60th percentile fee for transactions.
    pub p60: String,
    /// The 70th percentile fee for transactions.
    pub p70: String,
    /// The 80th percentile fee for transactions.
    pub p80: String,
    /// The 90th percentile fee for transactions.
    pub p90: String,
    /// The 95th percentile fee for transactions.
    pub p95: String,
    /// The 99th percentile fee for transactions.
    pub p99: String,
}

impl Response for FeeStatsResponse {
    fn from_json(json: String) -> Result<Self, String> {
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}