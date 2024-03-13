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

    #[test]
    fn dummy_test() {
        assert_eq!(super::FEE_STATS_PATH, "fee_stats");
    }

    #[tokio::test]
    async fn test_get_fee_stats() {
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let fee_stats_request = FeeStatsRequest::new();
        let _fee_stats_response = horizon_client.get_fee_stats(&fee_stats_request).await;

        assert!(_fee_stats_response.is_ok());

        // there is not much use in testing the values of the response, as they are subject to constant change
    }
}
