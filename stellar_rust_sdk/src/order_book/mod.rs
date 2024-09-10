pub mod details_request;
pub mod response;

static ORDER_BOOK_PATH: &str = "order_book";

pub mod prelude {
    pub use super::details_request::*;
    pub use super::response::*;
}

pub mod tests {


    #[tokio::test]
    async fn get_order_bookdetails() {

        use crate::models::prelude::*;
        use crate::horizon_client;
        use crate::order_book::prelude::DetailsRequest;

        const BIDS_N: &u32 = &1;
        const BIDS_D: &u32 = &5;
        const BIDS_PRICE: &str = "0.2000000";
        const ASKS_N: &u32 = &5;
        const ASKS_D: &u32 = &1;
        const ASKS_PRICE: &str = "5.0000000";
        const BASE_ASSET_TYPE: &str = "native";
        const BASE_ASSET_CODE: &str = "IOM";
        const BASE_ASSET_ISSUER: &str = "GCDE6MVFIOYF7YZCSVA6V7MDCFTNWMIOF5PQU3DWPH27AHNX4ERY6AKS";
        const COUNTER_ASSET_TYPE: &str = "credit_alphanum4";

        let horizon_client =
            horizon_client::HorizonClient::new("https://horizon-testnet.stellar.org".to_string())
                .unwrap();

        let details_request = DetailsRequest::new()
            .set_selling_asset(AssetType::Native)
            .unwrap()
            .set_buying_asset(AssetType::Alphanumeric4(AssetData {
                asset_code: "IOM".to_string(),
                asset_issuer: "GCDE6MVFIOYF7YZCSVA6V7MDCFTNWMIOF5PQU3DWPH27AHNX4ERY6AKS"
                    .to_string(),
            }))
            .unwrap();

        let details_response = horizon_client
            .get_order_book_details(&details_request)
            .await;

        assert!(details_response.is_ok());

        let binding = details_response.unwrap();

        assert_eq!(binding.bids()[0].price_ratio().numenator(), BIDS_N);
        assert_eq!(binding.bids()[0].price_ratio().denominator(), BIDS_D);
        assert_eq!(binding.bids()[0].price(), BIDS_PRICE);

        // The amount changes all the time
        assert_ne!(binding.bids()[0].amount(), "0");

        assert_eq!(binding.asks()[0].price_ratio().numenator(), ASKS_N);
        assert_eq!(binding.asks()[0].price_ratio().denominator(), ASKS_D);
        assert_eq!(binding.asks()[0].price(), ASKS_PRICE);
        // The amount changes all the time
        assert_ne!(binding.asks()[0].amount(), "0");

        assert_eq!(
            binding.base().asset_type().as_deref(),
            Some(BASE_ASSET_TYPE)
        );
        assert_eq!(binding.base().asset_code().as_deref(), None);
        assert_eq!(binding.base().asset_issuer().as_deref(), None);

        assert_eq!(
            binding.counter().asset_type().as_deref(),
            Some(COUNTER_ASSET_TYPE)
        );

        assert_eq!(
            binding.counter().asset_code().as_deref(),
            Some(BASE_ASSET_CODE)
        );

        assert_eq!(
            binding.counter().asset_issuer().as_deref(),
            Some(BASE_ASSET_ISSUER)
        );
    }
}
