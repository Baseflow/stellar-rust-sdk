pub mod details_request;
pub mod response;

static ORDER_BOOK_PATH: &str = "order_book";

mod prelude {
    pub use super::details_request::*;
    pub use super::response::*;
}

pub mod tests {
    use crate::{
        horizon_client,
        order_book::prelude::{Asset, AssetType, DetailsRequest},
    };

    #[tokio::test]
    async fn get_order_bookdetails() {
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

        let details_response = horizon_client.get_order_book_details(&details_request).await;
        
        details_response.unwrap();

        // assert!(details_response.is_ok());
    }
}
