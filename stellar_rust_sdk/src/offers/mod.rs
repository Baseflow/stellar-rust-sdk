/// Provides the `SingleOfferRequest`.
///
/// This module provides the `SingleOfferRequest` struct, specifically designed for
/// constructing requests to query information about a single offer from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_single_offer`](crate::horizon_client::HorizonClient::get_single_offer)
/// method.
///
pub mod single_offer_request;

/// Provides the `AllOffersRequest`.
///
/// This module provides the `AllOffersRequest` struct, specifically designed for
/// constructing requests to query information about all offers from the Horizon
/// server. It is tailored for use with the [`HorizonClient::get_all_offers`](crate::horizon_client::HorizonClient::get_all_offers)
/// method.
///
pub mod all_offers_request;

/// Provides the responses.
///
/// This module defines structures representing the response from the Horizon API when querying
/// for offers. The structures are designed to deserialize the JSON response into Rust
/// objects, enabling straightforward access to various details of a single Stellar account.
///
/// These structures are equipped with serialization capabilities to handle the JSON data from the
/// Horizon server and with getter methods for easy field access.
///
pub mod response;

/// The base path for offer-related endpoints in the Horizon API.
///
/// # Usage
/// This variable is intended to be used internally by the request-building logic
/// to ensure consistent and accurate path construction for offer-related API calls.
///
static OFFERS_PATH: &str = "offers";

/// The `prelude` module of the `offers` module.
///
/// This module serves as a convenience for users of the Horizon Rust SDK, allowing for easy and
/// ergonomic import of the most commonly used items across various modules. It re-exports
/// key structs and traits from the sibling modules, simplifying access to these components
/// when using the library.
///
/// By importing the contents of `prelude`, users can conveniently access the primary
/// functionalities of the offer-related modules without needing to import each item
/// individually.
///
/// # Contents
///
/// The `prelude` includes the following re-exports:
///
/// * From `single_offer_request`: All items (e.g. `SingleOfferRequest`).
/// * From `response`: All items (e.g. `SingleOfferResponse`, `PriceR`, etc.).
///
/// # Example
/// ```
/// # use crate::stellar_rs::models::*;
/// // Import the contents of the offers prelude
/// use stellar_rs::offers::prelude::*;
///
/// // Now you can directly use SingleOfferRequest, SingleOfferResponse, etc.
/// let single_offer_request = SingleOfferRequest::new();
/// ```
///
pub mod prelude {
    pub use super::single_offer_request::*;
    pub use super::response::*;
}

#[cfg(test)]
pub mod test {
    use super::prelude::*;
    use crate::horizon_client::HorizonClient;

    #[tokio::test]
    async fn test_get_single_offer() {
        const LINK_SELF: &str = "https://horizon-testnet.stellar.org/offers/1";
        const LINK_OFFER_MAKER: &str = "https://horizon-testnet.stellar.org/accounts/GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";
        const OFFER_ID: &str = "1";
        const PAGING_TOKEN: &str = "1";
        const SELLER: &str = "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";
        const SELLING_ASSET_TYPE: &str = "credit_alphanum4";
        const SELLING_ASSET_CODE: &str = "USDC";
        const SELLING_ASSET_ISSUER: &str = "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";
        const BUYING_ASSET_TYPE: &str = "credit_alphanum12";
        const BUYING_ASSET_CODE: &str = "USDCAllow";
        const BUYING_ASSET_ISSUER: &str = "GAWZGWFOURKXZ4XYXBGFADZM4QIG6BJNM74XIZCEIU3BHM62RN2MDEZN";
        const AMOUNT: &str = "909086990804.0875807";
        const PRICE_R_N: &u32 = &1;
        const PRICE_R_D: &u32 = &1;
        const PRICE: &str = "1.0000000";
        const LAST_MODIFIED_LEDGER: &u32 = &747543;
        const LAST_MODIFIED_TIME: &str = "2024-03-23T04:51:18Z";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org"
            .to_string())
            .unwrap();

        let single_offer_request =
            SingleOfferRequest::new()
            .set_offer_id(OFFER_ID.to_string())
            .unwrap();

        let single_offer_response = horizon_client
            .get_single_offer(&single_offer_request)
            .await;

        assert!(single_offer_response.clone().is_ok());
        let response = single_offer_response.unwrap();
        assert_eq!(response.links().self_link().href().as_ref().unwrap(), LINK_SELF);
        assert_eq!(response.links().offer_maker().href().as_ref().unwrap(), LINK_OFFER_MAKER);
        assert_eq!(response.id(), OFFER_ID);
        assert_eq!(response.paging_token(), PAGING_TOKEN);
        assert_eq!(response.seller(), SELLER);
        assert_eq!(response.selling().asset_type(), SELLING_ASSET_TYPE);
        assert_eq!(response.selling().asset_code().as_ref().unwrap(), SELLING_ASSET_CODE);
        assert_eq!(response.selling().asset_issuer().as_ref().unwrap(), SELLING_ASSET_ISSUER);
        assert_eq!(response.buying().asset_type(), BUYING_ASSET_TYPE);
        assert_eq!(response.buying().asset_code().as_ref().unwrap(), BUYING_ASSET_CODE);
        assert_eq!(response.buying().asset_issuer().as_ref().unwrap(), BUYING_ASSET_ISSUER);
        assert_eq!(response.amount(), AMOUNT);
        assert_eq!(response.price_ratio().numenator(), PRICE_R_N);
        assert_eq!(response.price_ratio().denominator(), PRICE_R_D);
        assert_eq!(response.price_decimal(), PRICE);
        assert_eq!(response.last_modified_ledger(), LAST_MODIFIED_LEDGER);
        assert_eq!(response.last_modified_time(), LAST_MODIFIED_TIME);
    }
}