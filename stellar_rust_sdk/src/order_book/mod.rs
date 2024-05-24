pub mod details_request;
pub mod response;

static ORDER_BOOK_PATH: &str = "order_book";

pub mod prelude {
    pub use super::details_request::*;
    pub use super::response::*;
}

pub mod tests {
    use crate::horizon_client;
    use crate::order_book::prelude::{Asset, AssetType, DetailsRequest};

    #[tokio::test]
    async fn get_order_bookdetails() {
        const BIDS_N: &u32 = &250000;
        const BIDS_D: &u32 = &21749;
        const BIDS_PRICE: &str = "11.4947814";
        const BIDS_AMOUNT: &str = "2556626.8467920";

        const ASKS_N: &u32 = &2299;
        const ASKS_D: &u32 = &200;
        const ASKS_PRICE: &str = "11.4950000";
        const ASKS_AMOUNT: &str = "162468.5993642";

        const BASE_ASSET_TYPE: &str = "credit_alphanum4";
        const BASE_ASSET_CODE: &str = "USDC";
        const BASE_ASSET_ISSUER: &str = "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";

        const COUNTER_ASSET_TYPE: &str = "native";

        let horizon_client =
            horizon_client::HorizonClient::new("https://horizon-testnet.stellar.org".to_string())
                .unwrap();

        let details_request = DetailsRequest::new()
            .set_buying_asset(AssetType::Native)
            .unwrap()
            .set_selling_asset(AssetType::Alphanumeric4(Asset {
                asset_code: "USDC".to_string(),
                asset_issuer: "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5"
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
        //assert_eq!(binding.bids()[0].amount(), BIDS_AMOUNT);

        assert_eq!(binding.asks()[0].price_ratio().numenator(), ASKS_N);
        assert_eq!(binding.asks()[0].price_ratio().denominator(), ASKS_D);
        assert_eq!(binding.asks()[0].price(), ASKS_PRICE);
        // The amount changes all the time
        assert_ne!(binding.asks()[0].amount(), "0");
        //assert_eq!(binding.asks()[0].amount(), ASKS_AMOUNT);

        assert_eq!(
            binding.base().asset_type().as_deref(),
            Some(BASE_ASSET_TYPE)
        );
        assert_eq!(
            binding.base().asset_code().as_deref(),
            Some(BASE_ASSET_CODE)
        );
        assert_eq!(
            binding.base().asset_issuer().as_deref(),
            Some(BASE_ASSET_ISSUER)
        );

        assert_eq!(
            binding.counter().asset_type().as_deref(),
            Some(COUNTER_ASSET_TYPE)
        );
    }
}
