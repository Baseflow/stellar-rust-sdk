/// Provides the `AllTradesRequest`.
///
/// This module provides the `AllTradesRequest` struct, specifically designed for
/// constructing requests to query information about all trades from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_all_trades`](crate::horizon_client::HorizonClient::get_all_trades)
/// method.
///
pub mod all_trades_request;

/// Provides the `TradesForAccountRequest`.
///
/// This module provides the `TradesForAccountRequest` struct, specifically designed for constructing requests to
/// retrieve all trades for a given account from the Horizon server. It is tailored for use with the
/// [`HorizonClient::get_trades_for_account`](crate::horizon_client::HorizonClient::get_trades_for_account) method.
///
pub mod trades_for_account_request;

/// Provides the `TradesForLiquidityPoolRequest`.
///
/// This module provides the `TradesForLiquidityPoolRequest` struct, specifically designed for constructing requests to
/// retrieve successful trades fulfilled by the given liquidity pool from the Horizon server. It is tailored for use with the
/// [`HorizonClient::get_trades_for_liquidity_pool`](crate::horizon_client::HorizonClient::get_trades_for_liquidity_pool) method.
///
pub mod trades_for_liquidity_pool_request;

/// Provides the `TradesForOfferRequest`.
///
/// This module provides the `TradesForOfferRequest` struct, specifically designed for constructing requests to
/// retrieve all trades for a given offer from the Horizon server. It is tailored for use with the
/// [`HorizonClient::get_trades_for_offer`](crate::horizon_client::HorizonClient::get_trades_for_offer) method.
///
pub mod trades_for_offer_request;

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
/// * From `all_trades_request`: All items (e.g. `AllTradesRequest`, `AssetType`, `AssetData`, etc.).
/// * From `response`: All items (e.g. `TradeResponse`, `AllTradesResponse`, etc.).
/// * From `trades_for_account_request`: All items (e.g. `TradesForAccountRequest`, `TradeAccountId`, etc.).
/// * From `trades_for_liquidity_pool_request`: All items (e.g. `TradesForLiquidityPoolRequest`, `TradeLiquidityPoolId`, etc.).
/// * From `trades_for_offer_request`: All items (e.g. `TradesForOfferRequest`, `TradeOfferId`, etc.).
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
    pub use super::all_trades_request::*;
    pub use super::response::*;
    pub use super::trades_for_account_request::*;
    pub use super::trades_for_liquidity_pool_request::*;
    pub use super::trades_for_offer_request::*;
}

#[cfg(test)]
pub mod test {
    use crate::{horizon_client::HorizonClient, trades::prelude::*};

    #[tokio::test]
    async fn all_trades_request() {
        const LINK_SELF: &str = "";
        const LINK_BASE: &str = "https://horizon-testnet.stellar.org/accounts/GCUOMNFW7YG55YHY5S5W7FE247PWODUDUZ4SOVZFEON47KZ7AXFG6D6A";
        const LINK_COUNTER: &str = "https://horizon-testnet.stellar.org/accounts/GBHRHA3KGRJBXBFER7VHI3WS5SKUXOP5TQ3YITVD7WJ2D3INGK62FZJR";
        const LINK_OPERATION: &str =
            "https://horizon-testnet.stellar.org/operations/23944442687489";
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
        const COUNTER_ASSET_ISSUER: &str =
            "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";
        const BASE_IS_SELLER: &bool = &true;
        const PRICE_N: &str = "3";
        const PRICE_R: &str = "10";

        let all_trades_request = AllTradesRequest::new();
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();
        let all_trades_response = horizon_client.get_all_trades(&all_trades_request).await;

        assert!(all_trades_response.clone().is_ok());
        let binding = all_trades_response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(
            response.links().self_link().href().as_ref().unwrap(),
            LINK_SELF
        );
        assert_eq!(response.links().base().href().as_ref().unwrap(), LINK_BASE);
        assert_eq!(
            response.links().counter().href().as_ref().unwrap(),
            LINK_COUNTER
        );
        assert_eq!(
            response.links().operation().href().as_ref().unwrap(),
            LINK_OPERATION
        );
        assert_eq!(response.id(), ID);
        assert_eq!(response.paging_token(), PAGING_TOKEN);
        assert_eq!(response.ledger_close_time(), LEDGER_CLOSE_TIME);
        assert_eq!(response.trade_type(), TRADE_TYPE);
        assert_eq!(response.base_offer_id().as_ref().unwrap(), BASE_OFFER_ID);
        assert_eq!(response.base_account().as_ref().unwrap(), BASE_ACCOUNT);
        assert_eq!(response.base_amount(), BASE_AMOUNT);
        assert_eq!(
            response.base_asset_type().as_ref().unwrap(),
            BASE_ASSET_TYPE
        );
        assert_eq!(
            response.base_asset_code().as_ref().unwrap(),
            BASE_ASSET_CODE
        );
        assert_eq!(
            response.base_asset_issuer().as_ref().unwrap(),
            BASE_ASSET_ISSUER
        );
        assert_eq!(
            response.counter_offer_id().as_ref().unwrap(),
            COUNTER_OFFER_ID
        );
        assert_eq!(
            response.counter_account().as_ref().unwrap(),
            COUNTER_ACCOUNT
        );
        assert_eq!(response.counter_amount(), COUNTER_AMOUNT);
        assert_eq!(
            response.counter_asset_type().as_ref().unwrap(),
            COUNTER_ASSET_TYPE
        );
        assert_eq!(
            response.counter_asset_code().as_ref().unwrap(),
            COUNTER_ASSET_CODE
        );
        assert_eq!(
            response.counter_asset_issuer().as_ref().unwrap(),
            COUNTER_ASSET_ISSUER
        );
        assert_eq!(response.base_is_seller(), BASE_IS_SELLER);
        assert_eq!(response.price().as_ref().unwrap().numenator(), PRICE_N);
        assert_eq!(response.price().as_ref().unwrap().denominator(), PRICE_R);
    }

    #[tokio::test]
    async fn trades_for_account_request() {
        const ACCOUNT_ID: &str = "GCUOMNFW7YG55YHY5S5W7FE247PWODUDUZ4SOVZFEON47KZ7AXFG6D6A"; // ID for the request
        const LINK_SELF: &str = "";
        const LINK_BASE: &str = "https://horizon-testnet.stellar.org/accounts/GCUOMNFW7YG55YHY5S5W7FE247PWODUDUZ4SOVZFEON47KZ7AXFG6D6A";
        const LINK_COUNTER: &str = "https://horizon-testnet.stellar.org/accounts/GBHRHA3KGRJBXBFER7VHI3WS5SKUXOP5TQ3YITVD7WJ2D3INGK62FZJR";
        const LINK_OPERATION: &str =
            "https://horizon-testnet.stellar.org/operations/23944442687489";
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
        const COUNTER_ASSET_ISSUER: &str =
            "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";
        const BASE_IS_SELLER: &bool = &true;
        const PRICE_N: &str = "3";
        const PRICE_R: &str = "10";

        let trades_for_account_request = TradesForAccountRequest::new()
            .set_account_id(ACCOUNT_ID.to_string())
            .unwrap();
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();
        let trades_for_account_response = horizon_client
            .get_trades_for_account(&trades_for_account_request)
            .await;

        assert!(trades_for_account_response.clone().is_ok());
        let binding = trades_for_account_response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(
            response.links().self_link().href().as_ref().unwrap(),
            LINK_SELF
        );
        assert_eq!(response.links().base().href().as_ref().unwrap(), LINK_BASE);
        assert_eq!(
            response.links().counter().href().as_ref().unwrap(),
            LINK_COUNTER
        );
        assert_eq!(
            response.links().operation().href().as_ref().unwrap(),
            LINK_OPERATION
        );
        assert_eq!(response.id(), ID);
        assert_eq!(response.paging_token(), PAGING_TOKEN);
        assert_eq!(response.ledger_close_time(), LEDGER_CLOSE_TIME);
        assert_eq!(response.trade_type(), TRADE_TYPE);
        assert_eq!(response.base_offer_id().as_ref().unwrap(), BASE_OFFER_ID);
        assert_eq!(response.base_account().as_ref().unwrap(), BASE_ACCOUNT);
        assert_eq!(response.base_amount(), BASE_AMOUNT);
        assert_eq!(
            response.base_asset_type().as_ref().unwrap(),
            BASE_ASSET_TYPE
        );
        assert_eq!(
            response.base_asset_code().as_ref().unwrap(),
            BASE_ASSET_CODE
        );
        assert_eq!(
            response.base_asset_issuer().as_ref().unwrap(),
            BASE_ASSET_ISSUER
        );
        assert_eq!(
            response.counter_offer_id().as_ref().unwrap(),
            COUNTER_OFFER_ID
        );
        assert_eq!(
            response.counter_account().as_ref().unwrap(),
            COUNTER_ACCOUNT
        );
        assert_eq!(response.counter_amount(), COUNTER_AMOUNT);
        assert_eq!(
            response.counter_asset_type().as_ref().unwrap(),
            COUNTER_ASSET_TYPE
        );
        assert_eq!(
            response.counter_asset_code().as_ref().unwrap(),
            COUNTER_ASSET_CODE
        );
        assert_eq!(
            response.counter_asset_issuer().as_ref().unwrap(),
            COUNTER_ASSET_ISSUER
        );
        assert_eq!(response.base_is_seller(), BASE_IS_SELLER);
        assert_eq!(response.price().as_ref().unwrap().numenator(), PRICE_N);
        assert_eq!(response.price().as_ref().unwrap().denominator(), PRICE_R);
    }

    #[tokio::test]
    async fn trades_for_liquidity_pools_request() {
        const LIQUIDITY_POOL_ID: &str =
            "0b3c88caa5aeada296646c1810893e3b04cba0426cff8ff6a63cf6f35cc7f5b3"; // ID for the request
        const LINK_SELF: &str = "";
        const LINK_BASE: &str = "https://horizon-testnet.stellar.org/liquidity_pools/0b3c88caa5aeada296646c1810893e3b04cba0426cff8ff6a63cf6f35cc7f5b3";
        const LINK_COUNTER: &str = "https://horizon-testnet.stellar.org/accounts/GAV5JC25XAB4ALRUDNPB6TZMHSNXFFONKGMLRKHBC5KYGXOGXEVE2BOW";
        const LINK_OPERATION: &str =
            "https://horizon-testnet.stellar.org/operations/1110815981719553";
        const ID: &str = "1110815981719553-0";
        const PAGING_TOKEN: &str = "1110815981719553-0";
        const LEDGER_CLOSE_TIME: &str = "2024-06-27T14:28:37Z";
        const TRADE_TYPE: &str = "liquidity_pool";
        const LIQUIDITY_POOL_FEE_BP: &u32 = &30;
        const BASE_LIQUIDITY_POOL_ID: &str =
            "0b3c88caa5aeada296646c1810893e3b04cba0426cff8ff6a63cf6f35cc7f5b3";
        const BASE_AMOUNT: &str = "9.3486278";
        const BASE_ASSET_TYPE: &str = "credit_alphanum12";
        const BASE_ASSET_CODE: &str = "FLUTTER";
        const BASE_ASSET_ISSUER: &str = "GCGTOQSNERFVVJ6Y7YZYDF3MTZIY63KIEFMKA26Q7YPV3AFYD2JSRNYN";
        const COUNTER_OFFER_ID: &str = "4612796834409107457";
        const COUNTER_ACCOUNT: &str = "GAV5JC25XAB4ALRUDNPB6TZMHSNXFFONKGMLRKHBC5KYGXOGXEVE2BOW";
        const COUNTER_AMOUNT: &str = "10.0000000";
        const COUNTER_ASSET_TYPE: &str = "credit_alphanum4";
        const COUNTER_ASSET_CODE: &str = "SDK";
        const COUNTER_ASSET_ISSUER: &str =
            "GAGTRBIF75N7NUA37JGGJZKXIS4JJKTQERRFWTP5DN4SM4OC2T6QPMQB";
        const BASE_IS_SELLER: &bool = &true;
        const PRICE_N: &str = "100000000";
        const PRICE_D: &str = "93486278";

        let trades_for_liquidity_pool_request = TradesForLiquidityPoolRequest::new()
            .set_liquidity_pool_id(LIQUIDITY_POOL_ID.to_string())
            .unwrap();
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();
        let trades_for_liquidity_pool_response = horizon_client
            .get_trades_for_liquidity_pool(&trades_for_liquidity_pool_request)
            .await;

        // assert!(trades_for_liquidity_pool_response.clone().is_ok());
        let binding = trades_for_liquidity_pool_response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(
            response.links().self_link().href().as_ref().unwrap(),
            LINK_SELF
        );
        assert_eq!(response.links().base().href().as_ref().unwrap(), LINK_BASE);
        assert_eq!(
            response.links().counter().href().as_ref().unwrap(),
            LINK_COUNTER
        );
        assert_eq!(
            response.links().operation().href().as_ref().unwrap(),
            LINK_OPERATION
        );
        assert_eq!(response.id(), ID);
        assert_eq!(response.paging_token(), PAGING_TOKEN);
        assert_eq!(response.ledger_close_time(), LEDGER_CLOSE_TIME);
        assert_eq!(response.trade_type(), TRADE_TYPE);
        assert_eq!(
            response.liquidity_pool_fee_bp().as_ref().unwrap(),
            LIQUIDITY_POOL_FEE_BP
        );
        assert_eq!(
            response.base_liquidity_pool_id().as_ref().unwrap(),
            BASE_LIQUIDITY_POOL_ID
        );
        assert_eq!(response.base_amount(), BASE_AMOUNT);
        assert_eq!(
            response.base_asset_type().as_ref().unwrap(),
            BASE_ASSET_TYPE
        );
        assert_eq!(
            response.base_asset_code().as_ref().unwrap(),
            BASE_ASSET_CODE
        );
        assert_eq!(
            response.base_asset_issuer().as_ref().unwrap(),
            BASE_ASSET_ISSUER
        );
        assert_eq!(
            response.counter_offer_id().as_ref().unwrap(),
            COUNTER_OFFER_ID
        );
        assert_eq!(
            response.counter_account().as_ref().unwrap(),
            COUNTER_ACCOUNT
        );
        assert_eq!(response.counter_amount(), COUNTER_AMOUNT);
        assert_eq!(
            response.counter_asset_type().as_ref().unwrap(),
            COUNTER_ASSET_TYPE
        );
        assert_eq!(
            response.counter_asset_code().as_ref().unwrap(),
            COUNTER_ASSET_CODE
        );
        assert_eq!(
            response.counter_asset_issuer().as_ref().unwrap(),
            COUNTER_ASSET_ISSUER
        );
        assert_eq!(response.base_is_seller(), BASE_IS_SELLER);
        assert_eq!(response.price().as_ref().unwrap().numenator(), PRICE_N);
        assert_eq!(response.price().as_ref().unwrap().denominator(), PRICE_D);
    }

    #[tokio::test]
    async fn trades_for_offers_request() {
        const OFFER_ID: &str = "0"; // ID for the request
        const LINK_SELF: &str = "";
        const LINK_BASE: &str = "https://horizon-testnet.stellar.org/accounts/GCUOMNFW7YG55YHY5S5W7FE247PWODUDUZ4SOVZFEON47KZ7AXFG6D6A";
        const LINK_COUNTER: &str = "https://horizon-testnet.stellar.org/accounts/GBHRHA3KGRJBXBFER7VHI3WS5SKUXOP5TQ3YITVD7WJ2D3INGK62FZJR";
        const LINK_OPERATION: &str =
            "https://horizon-testnet.stellar.org/operations/23944442687489";
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
        const COUNTER_ACCOUNT: &str = "GBHRHA3KGRJBXBFER7VHI3WS5SKUXOP5TQ3YITVD7WJ2D3INGK62FZJR";
        const COUNTER_AMOUNT: &str = "1.0800000";
        const COUNTER_ASSET_TYPE: &str = "credit_alphanum4";
        const COUNTER_ASSET_CODE: &str = "XUSD";
        const COUNTER_ASSET_ISSUER: &str =
            "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI";
        const BASE_IS_SELLER: &bool = &true;
        const PRICE_N: &str = "3";
        const PRICE_D: &str = "10";

        let trades_for_offer_request = TradesForOfferRequest::new()
            .set_offer_id(OFFER_ID.to_string())
            .unwrap();
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();
        let trades_for_liquidity_pools_response = horizon_client
            .get_trades_for_offer(&trades_for_offer_request)
            .await;

        assert!(trades_for_liquidity_pools_response.clone().is_ok());
        let binding = trades_for_liquidity_pools_response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(
            response.links().self_link().href().as_ref().unwrap(),
            LINK_SELF
        );
        assert_eq!(response.links().base().href().as_ref().unwrap(), LINK_BASE);
        assert_eq!(
            response.links().counter().href().as_ref().unwrap(),
            LINK_COUNTER
        );
        assert_eq!(
            response.links().operation().href().as_ref().unwrap(),
            LINK_OPERATION
        );
        assert_eq!(response.id(), ID);
        assert_eq!(response.paging_token(), PAGING_TOKEN);
        assert_eq!(response.ledger_close_time(), LEDGER_CLOSE_TIME);
        assert_eq!(response.trade_type(), TRADE_TYPE);
        assert_eq!(response.base_offer_id().as_ref().unwrap(), BASE_OFFER_ID);
        assert_eq!(response.base_account().as_ref().unwrap(), BASE_ACCOUNT);
        assert_eq!(response.base_amount(), BASE_AMOUNT);
        assert_eq!(
            response.base_asset_type().as_ref().unwrap(),
            BASE_ASSET_TYPE
        );
        assert_eq!(
            response.base_asset_code().as_ref().unwrap(),
            BASE_ASSET_CODE
        );
        assert_eq!(
            response.base_asset_issuer().as_ref().unwrap(),
            BASE_ASSET_ISSUER
        );
        assert_eq!(
            response.counter_account().as_ref().unwrap(),
            COUNTER_ACCOUNT
        );
        assert_eq!(response.counter_amount(), COUNTER_AMOUNT);
        assert_eq!(
            response.counter_asset_type().as_ref().unwrap(),
            COUNTER_ASSET_TYPE
        );
        assert_eq!(
            response.counter_asset_code().as_ref().unwrap(),
            COUNTER_ASSET_CODE
        );
        assert_eq!(
            response.counter_asset_issuer().as_ref().unwrap(),
            COUNTER_ASSET_ISSUER
        );
        assert_eq!(response.base_is_seller(), BASE_IS_SELLER);
        assert_eq!(response.price().as_ref().unwrap().numenator(), PRICE_N);
        assert_eq!(response.price().as_ref().unwrap().denominator(), PRICE_D);
    }
}
