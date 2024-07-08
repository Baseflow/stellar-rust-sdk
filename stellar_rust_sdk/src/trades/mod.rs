/// Provides the `AllTradesRequest`.
///
/// This module provides the `AllTradesRequest` struct, specifically designed for
/// constructing requests to query information about all trades from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_all_trades`](crate::horizon_client::HorizonClient::get_all_trades)
/// method.
///
pub mod all_trades_request;

// TODO: Documentation
pub mod trades_for_account_request;

/// Provides the responses.
///
/// This module defines structures representing the response from the Horizon API when querying
/// for trades. The structures are designed to deserialize the JSON response into Rust
/// objects, enabling straightforward access to various details of a single trade.
///
/// These structures are equipped with serialization capabilities to handle the JSON data from the
/// Horizon server and with getter methods for easy field access.
///
pub mod response;

/// The base path for trade-related endpoints in the Horizon API.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for trade-related API calls.
///
static TRADES_PATH: &str = "trades";

/// The `prelude` module of the `trades` module.
///
/// This module serves as a convenience for users of the Horizon Rust SDK, allowing for easy and
/// ergonomic import of the most commonly used items across various modules. It re-exports
/// key structs and traits from the sibling modules, simplifying access to these components
/// when using the library.
///
/// By importing the contents of `prelude`, users can conveniently access the primary
/// functionalities of the trade-related modules without needing to import each item
/// individually.
///
/// # Contents
///
/// The `prelude` includes the following re-exports:
///
/// * From `all_trades_request`: All items (e.g. `AllTradesRequest`).
/// * From `trades_for_account_request`: All items (e.g. `TradesForAccountRequest`).
/// * From `response`: All items (e.g. `TradeResponse`, `AllTradesResponse`, etc.).
///
/// # Example
/// ```
/// # use crate::stellar_rs::models::*;
/// // Import the contents of the trades prelude
/// use stellar_rs::trades::prelude::*;
///
/// // Now you can directly use AllTradesRequest, AllTradesResponse, etc.
/// let all_trades_request = AllTradesRequest::new();
/// ```
///
pub mod prelude {
    pub use super::trades_for_account_request::*;
    pub use super::all_trades_request::*;
    pub use super::response::*;
}

#[cfg(test)]
pub mod test {
    use crate::{trades::prelude::*, horizon_client::HorizonClient};

    #[tokio::test]
    async fn all_trades_request() {
        const LINK_SELF: &str = "";
        const LINK_BASE: &str = "https://horizon-testnet.stellar.org/accounts/GCUOMNFW7YG55YHY5S5W7FE247PWODUDUZ4SOVZFEON47KZ7AXFG6D6A";
        const LINK_COUNTER: &str = "https://horizon-testnet.stellar.org/accounts/GBHRHA3KGRJBXBFER7VHI3WS5SKUXOP5TQ3YITVD7WJ2D3INGK62FZJR";
        const LINK_OPERATION: &str = "https://horizon-testnet.stellar.org/operations/23944442687489";
        const ID: &str = "23944442687489-0";
        const PAGING_TOKEN: &str = "23944442687489-0";
        const LEDGER_CLOSE_TIME: &str = "2024-06-12T04:58:59Z";
        const TRADE_TYPE: &str = "orderbook";
        const BASE_OFFER_ID: &str = "20";
        const BASE_ACCOUNT: &str = "GCUOMNFW7YG55YHY5S5W7FE247PWODUDUZ4SOVZFEON47KZ7AXFG6D6A";
        const BASE_AMOUNT: &str = "3.6000000";
        const BASE_ASSET_TYPE: &str = "credit_alphanum4";
        const BASE_ASSET_CODE: &str = "XETH";
        const BASE_ASSET_ISSUER: &str = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";
        const COUNTER_OFFER_ID: &str = "21";
        const COUNTER_ACCOUNT: &str = "GBHRHA3KGRJBXBFER7VHI3WS5SKUXOP5TQ3YITVD7WJ2D3INGK62FZJR";
        const COUNTER_AMOUNT: &str = "1.0800000";
        const COUNTER_ASSET_TYPE: &str = "credit_alphanum4";
        const COUNTER_ASSET_CODE: &str = "XUSD";
        const COUNTER_ASSET_ISSUER: &str = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";
        const BASE_IS_SELLER: &bool = &true;
        const PRICE_N: &str = "3";
        const PRICE_R: &str = "10";

        let all_trades_request = AllTradesRequest::new();
        
        let horizon_client =
        HorizonClient::new("https://horizon-testnet.stellar.org"
            .to_string())
            .unwrap();

        let all_trades_response = horizon_client
            .get_all_trades(&all_trades_request)
            .await;

        assert!(all_trades_response.clone().is_ok());
        let binding = all_trades_response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(response.links().self_link().href().as_ref().unwrap(), LINK_SELF);
        assert_eq!(response.links().base().href().as_ref().unwrap(), LINK_BASE);
        assert_eq!(response.links().counter().href().as_ref().unwrap(), LINK_COUNTER);
        assert_eq!(response.links().operation().href().as_ref().unwrap(), LINK_OPERATION);
        assert_eq!(response.id(), ID);
        assert_eq!(response.paging_token(), PAGING_TOKEN);
        assert_eq!(response.ledger_close_time(), LEDGER_CLOSE_TIME);
        assert_eq!(response.trade_type(), TRADE_TYPE);
        assert_eq!(response.base_offer_id(), BASE_OFFER_ID);
        assert_eq!(response.base_account(), BASE_ACCOUNT);
        assert_eq!(response.base_amount(), BASE_AMOUNT);
        assert_eq!(response.base_asset_type().as_ref().unwrap(), BASE_ASSET_TYPE);
        assert_eq!(response.base_asset_code().as_ref().unwrap(), BASE_ASSET_CODE);
        assert_eq!(response.base_asset_issuer().as_ref().unwrap(), BASE_ASSET_ISSUER);
        assert_eq!(response.counter_offer_id(), COUNTER_OFFER_ID);
        assert_eq!(response.counter_account(), COUNTER_ACCOUNT);
        assert_eq!(response.counter_amount(), COUNTER_AMOUNT);
        assert_eq!(response.counter_asset_type().as_ref().unwrap(), COUNTER_ASSET_TYPE);
        assert_eq!(response.counter_asset_code().as_ref().unwrap(), COUNTER_ASSET_CODE);
        assert_eq!(response.counter_asset_issuer().as_ref().unwrap(), COUNTER_ASSET_ISSUER);
        assert_eq!(response.base_is_seller(), BASE_IS_SELLER);
        assert_eq!(response.price().as_ref().unwrap().numenator(), PRICE_N);
        assert_eq!(response.price().as_ref().unwrap().denominator(), PRICE_R);
        }

    #[tokio::test]
    async fn trades_for_account_request() {
        const ACCOUNT_ID: &str =  "GCUOMNFW7YG55YHY5S5W7FE247PWODUDUZ4SOVZFEON47KZ7AXFG6D6A";
        const LINK_SELF: &str = "";
        const LINK_BASE: &str = "https://horizon-testnet.stellar.org/accounts/GCUOMNFW7YG55YHY5S5W7FE247PWODUDUZ4SOVZFEON47KZ7AXFG6D6A";
        const LINK_COUNTER: &str = "https://horizon-testnet.stellar.org/accounts/GBHRHA3KGRJBXBFER7VHI3WS5SKUXOP5TQ3YITVD7WJ2D3INGK62FZJR";
        const LINK_OPERATION: &str = "https://horizon-testnet.stellar.org/operations/23944442687489";
        const ID: &str = "23944442687489-0";
        const PAGING_TOKEN: &str = "23944442687489-0";
        const LEDGER_CLOSE_TIME: &str = "2024-06-12T04:58:59Z";
        const TRADE_TYPE: &str = "orderbook";
        const BASE_OFFER_ID: &str = "20";
        const BASE_ACCOUNT: &str = "GCUOMNFW7YG55YHY5S5W7FE247PWODUDUZ4SOVZFEON47KZ7AXFG6D6A";
        const BASE_AMOUNT: &str = "3.6000000";
        const BASE_ASSET_TYPE: &str = "credit_alphanum4";
        const BASE_ASSET_CODE: &str = "XETH";
        const BASE_ASSET_ISSUER: &str = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";
        const COUNTER_OFFER_ID: &str = "21";
        const COUNTER_ACCOUNT: &str = "GBHRHA3KGRJBXBFER7VHI3WS5SKUXOP5TQ3YITVD7WJ2D3INGK62FZJR";
        const COUNTER_AMOUNT: &str = "1.0800000";
        const COUNTER_ASSET_TYPE: &str = "credit_alphanum4";
        
        const COUNTER_ASSET_CODE: &str = "XUSD";
        const COUNTER_ASSET_ISSUER: &str = "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";
        const BASE_IS_SELLER: &bool = &true;
        const PRICE_N: &str = "3";
        const PRICE_R: &str = "10";

        let trades_for_account_request = TradesForAccountRequest::new()
            .set_account_id(ACCOUNT_ID.to_string())
            .unwrap();
    
        let horizon_client =
        HorizonClient::new("https://horizon-testnet.stellar.org"
            .to_string())
            .unwrap();

        let trades_for_account_response = horizon_client
            .get_trades_for_account(&trades_for_account_request)
            .await;
    
        assert!(trades_for_account_response.clone().is_ok());
        let binding = trades_for_account_response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(response.links().self_link().href().as_ref().unwrap(), LINK_SELF);
        assert_eq!(response.links().base().href().as_ref().unwrap(), LINK_BASE);
        assert_eq!(response.links().counter().href().as_ref().unwrap(), LINK_COUNTER);
        assert_eq!(response.links().operation().href().as_ref().unwrap(), LINK_OPERATION);
        assert_eq!(response.id(), ID);
        assert_eq!(response.paging_token(), PAGING_TOKEN);
        assert_eq!(response.ledger_close_time(), LEDGER_CLOSE_TIME);
        assert_eq!(response.trade_type(), TRADE_TYPE);
        assert_eq!(response.base_offer_id(), BASE_OFFER_ID);
        assert_eq!(response.base_account(), BASE_ACCOUNT);
        assert_eq!(response.base_amount(), BASE_AMOUNT);
        assert_eq!(response.base_asset_type().as_ref().unwrap(), BASE_ASSET_TYPE);
        assert_eq!(response.base_asset_code().as_ref().unwrap(), BASE_ASSET_CODE);
        assert_eq!(response.base_asset_issuer().as_ref().unwrap(), BASE_ASSET_ISSUER);
        assert_eq!(response.counter_offer_id(), COUNTER_OFFER_ID);
        assert_eq!(response.counter_account(), COUNTER_ACCOUNT);
        assert_eq!(response.counter_amount(), COUNTER_AMOUNT);
        assert_eq!(response.counter_asset_type().as_ref().unwrap(), COUNTER_ASSET_TYPE);
        assert_eq!(response.counter_asset_code().as_ref().unwrap(), COUNTER_ASSET_CODE);
        assert_eq!(response.counter_asset_issuer().as_ref().unwrap(), COUNTER_ASSET_ISSUER);
        assert_eq!(response.base_is_seller(), BASE_IS_SELLER);
        assert_eq!(response.price().as_ref().unwrap().numenator(), PRICE_N);
        assert_eq!(response.price().as_ref().unwrap().denominator(), PRICE_R);
        }
}