pub mod fee_stats_request;
pub mod response;

static FEE_STATS_PATH: &str = "fee_stats";

pub mod prelude {
    pub use super::fee_stats_request::*;
    pub use super::response::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use crate::horizon_client::HorizonClient;

    #[tokio::test]
    async fn test_get_fee_stats() {
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let fee_stats_request = FeeStatsRequest::new();
        let fee_stats_response = horizon_client.get_fee_stats(&fee_stats_request).await;

        assert!(fee_stats_response.is_ok());

        let binding = fee_stats_response.unwrap();
        assert_eq!(binding.last_ledger().is_empty(), false);
        assert_eq!(binding.last_ledger_base_fee().is_empty(), false);
        assert_eq!(binding.ledger_capacity_usage().is_empty(), false);
        assert_eq!(binding.fee_charged().max().is_empty(), false);
        assert_eq!(binding.fee_charged().min().is_empty(), false);
        assert_eq!(binding.fee_charged().mode().is_empty(), false);
        assert_eq!(binding.fee_charged().p10().is_empty(), false);
        assert_eq!(binding.fee_charged().p20().is_empty(), false);
        assert_eq!(binding.fee_charged().p30().is_empty(), false);
        assert_eq!(binding.fee_charged().p40().is_empty(), false);
        assert_eq!(binding.fee_charged().p50().is_empty(), false);
        assert_eq!(binding.fee_charged().p60().is_empty(), false);
        assert_eq!(binding.fee_charged().p70().is_empty(), false);
        assert_eq!(binding.fee_charged().p80().is_empty(), false);
        assert_eq!(binding.fee_charged().p90().is_empty(), false);
        assert_eq!(binding.fee_charged().p95().is_empty(), false);
        assert_eq!(binding.fee_charged().p99().is_empty(), false);
        assert_eq!(binding.max_fee().max().is_empty(), false);
        assert_eq!(binding.max_fee().min().is_empty(), false);
        assert_eq!(binding.max_fee().mode().is_empty(), false);
        assert_eq!(binding.max_fee().p10().is_empty(), false);
        assert_eq!(binding.max_fee().p20().is_empty(), false);
        assert_eq!(binding.max_fee().p30().is_empty(), false);
        assert_eq!(binding.max_fee().p40().is_empty(), false);
        assert_eq!(binding.max_fee().p50().is_empty(), false);
        assert_eq!(binding.max_fee().p60().is_empty(), false);
        assert_eq!(binding.max_fee().p70().is_empty(), false);
        assert_eq!(binding.max_fee().p80().is_empty(), false);
        assert_eq!(binding.max_fee().p90().is_empty(), false);
        assert_eq!(binding.max_fee().p95().is_empty(), false);
        assert_eq!(binding.max_fee().p99().is_empty(), false);
    }
}
