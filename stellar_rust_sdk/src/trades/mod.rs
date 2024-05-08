// TODO: documentation
pub mod all_trades_request;

// TODO: documentation
static TRADES_PATH: &str = "trades";

// TODO: documentation
pub mod prelude {
    pub use super::all_trades_request::*;
}

// TODO: Write more tests.
#[cfg(test)]
pub mod test {
    use crate::trades::prelude::*;
    use crate::models::*;

    #[tokio::test]
    async fn create_request() {
        // This is just to test if the creating of requests with several combinations of contents works as expected.
        // TODO: Replace debug prints with asserts and use static/const.
        let request = AllTradesRequest::new()
            .set_counter_asset(AssetType::Alphanumeric4(AssetData {
                asset_code: "USDC".to_string(),
                asset_issuer: "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5"
                    .to_string(),
            }))
            .unwrap();

        println!("{:?}", request.base_asset);
        println!("{:?}", request.counter_asset);
        println!("{:?}", request.get_query_parameters());

        let request = AllTradesRequest::new();
        println!();
        println!("{:?}", request.base_asset);
        println!("{:?}", request.counter_asset);
        println!("{:?}", request.get_query_parameters());

        }
}