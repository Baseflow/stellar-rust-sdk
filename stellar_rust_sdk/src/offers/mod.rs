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

/// Provides the `OffersForAccountRequest`.
///
/// This module provides the `OffersForAccountRequest` struct, specifically designed for
/// constructing requests to query information about all offers a given account has 
/// currently open from the Horizon server. It is tailored for use with the 
/// [`HorizonClient::get_offers_for_account`](crate::horizon_client::HorizonClient::get_offers_for_account) method.
///
pub mod offers_for_account_request;

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
pub(crate) static OFFERS_PATH: &str = "offers";

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
/// * From `all_offers_request`: All items (e.g. `AllOffersRequest`).
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
    pub use super::all_offers_request::*;
    pub use super::offers_for_account_request::*;
    pub use super::response::*;
}

#[cfg(test)]
pub mod test {
    use super::prelude::*;
    use crate::{horizon_client::HorizonClient, models::*, Paginatable};

    const LINK_SELF: &str = "https://horizon-testnet.stellar.org/offers/7";
    const LINK_OFFER_MAKER: &str =  "https://horizon-testnet.stellar.org/accounts/GCXRNJ23TEHRNXQJEYXGQ3IYGVAWWY6Z2VOOWPP6STTYQCKXIRTNCN3E";
    const OFFER_ID: &str = "7";
    const PAGING_TOKEN: &str = "7";
    const SELLER: &str = "GCXRNJ23TEHRNXQJEYXGQ3IYGVAWWY6Z2VOOWPP6STTYQCKXIRTNCN3E";
    const SELLING_ASSET_TYPE: &str = "credit_alphanum12";
    const SELLING_ASSET_CODE: &str = "MBAUDD";
    const SELLING_ASSET_ISSUER: &str = "GD2YNRNSJ3EOFJAYGLKGKSIOLX2VU3UFDW3YFNOYMAHB26AEHSZBJU4U";
    const BUYING_ASSET_TYPE: &str = "credit_alphanum12";
    const BUYING_ASSET_CODE: &str = "TMB001128";
    const BUYING_ASSET_ISSUER: &str = "GBH2HB7DZN7PRJP5RED2SQZAKSYYBH43PQCQH3NOYT2Y2KLODQZM3M2F";
    const AMOUNT: &str = "41011.9400000";
    const PRICE_R_N: &u32 = &50;
    const PRICE_R_D: &u32 = &467;
    const PRICE: &str = "0.1070664";
    const LAST_MODIFIED_LEDGER: &u32 = &4739;
    const LAST_MODIFIED_TIME: &str = "2024-06-12T03:45:47Z";

    #[tokio::test]
    async fn test_get_single_offer() {
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

    #[tokio::test]
    async fn test_get_all_offers() {
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org"
            .to_string())
            .unwrap();

        // Create a request with no (optional) filters.
        let all_offers_request =
            AllOffersRequest::new();

        let all_offers_response = horizon_client
            .get_all_offers(&all_offers_request)
            .await;

        assert!(all_offers_response.clone().is_ok());
        let binding = all_offers_response.unwrap();
        let record = &binding.embedded().records()[0];
        assert_eq!(record.links().self_link().href().as_ref().unwrap(), LINK_SELF);
        assert_eq!(record.links().offer_maker().href().as_ref().unwrap(), LINK_OFFER_MAKER);
        assert_eq!(record.id(), OFFER_ID);
        assert_eq!(record.paging_token(), PAGING_TOKEN);
        assert_eq!(record.seller(), SELLER);
        assert_eq!(record.selling().asset_type(), SELLING_ASSET_TYPE);
        assert_eq!(record.selling().asset_code().as_ref().unwrap(), SELLING_ASSET_CODE);
        assert_eq!(record.selling().asset_issuer().as_ref().unwrap(), SELLING_ASSET_ISSUER);
        assert_eq!(record.buying().asset_type(), BUYING_ASSET_TYPE);
        assert_eq!(record.buying().asset_code().as_ref().unwrap(), BUYING_ASSET_CODE);
        assert_eq!(record.buying().asset_issuer().as_ref().unwrap(), BUYING_ASSET_ISSUER);
        assert_eq!(record.amount(), AMOUNT);
        assert_eq!(record.price_ratio().numenator(), PRICE_R_N);
        assert_eq!(record.price_ratio().denominator(), PRICE_R_D);
        assert_eq!(record.price_decimal(), PRICE);
        assert_eq!(record.last_modified_ledger(), LAST_MODIFIED_LEDGER);
        assert_eq!(record.last_modified_time(), LAST_MODIFIED_TIME);
    }

    #[tokio::test]
    async fn test_get_all_offers_filter() {
        // Different values are expected for this specific request.
        const LINK_OFFER_MAKER: &str = "https://horizon-testnet.stellar.org/accounts/GB3Q6QDZYTHWT7E5PVS3W7FUT5GVAFC5KSZFFLPU25GO7VTC3NM2ZTVO";
        const OFFER_ID: &str = "150";
        const PAGING_TOKEN: &str = "150";
        const SELLER: &str = "GB3Q6QDZYTHWT7E5PVS3W7FUT5GVAFC5KSZFFLPU25GO7VTC3NM2ZTVO";
        const SELLING_ASSET_TYPE: &str = "credit_alphanum4";
        const SELLING_ASSET_CODE: &str = "EURC";
        const SELLING_ASSET_ISSUER: &str = "GB3Q6QDZYTHWT7E5PVS3W7FUT5GVAFC5KSZFFLPU25GO7VTC3NM2ZTVO";
        const BUYING_ASSET_TYPE: &str = "credit_alphanum12";
        const BUYING_ASSET_CODE: &str = "EURCAllow";
        const BUYING_ASSET_ISSUER: &str = "GA6HVGLFUF3BHHGR5CMYXIVZ3RYVUH5EUYAOAY4T3OKI5OQVIWVRK24R";
        const AMOUNT: &str = "922307928093.4475807";
        const PRICE_R_N: &u32 = &1;
        const PRICE_R_D: &u32 = &1;
        const PRICE: &str = "1.0000000";
        const LAST_MODIFIED_LEDGER: &u32 = &286496;
        const LAST_MODIFIED_TIME: &str = "2024-06-29T07:08:23Z";
    
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org"
            .to_string())
            .unwrap();

        // Create a request and supply values for optional filters.
        let all_offers_request =
            AllOffersRequest::new()
            .set_seller(SELLER.to_string()).unwrap()
            .set_cursor(1).unwrap()
            .set_limit(100).unwrap()
            .set_order(Order::Asc).unwrap();

        let all_offers_response = horizon_client
            .get_all_offers(&all_offers_request)
            .await;

        assert!(all_offers_response.clone().is_ok());
        let binding = all_offers_response.unwrap();
        let record = &binding.embedded().records()[0];
        assert_eq!(record.links().offer_maker().href().as_ref().unwrap(), LINK_OFFER_MAKER);
        assert_eq!(record.id(), OFFER_ID);
        assert_eq!(record.paging_token(), PAGING_TOKEN);
        assert_eq!(record.seller(), SELLER);
        assert_eq!(record.selling().asset_type(), SELLING_ASSET_TYPE);
        assert_eq!(record.selling().asset_code().as_ref().unwrap(), SELLING_ASSET_CODE);
        assert_eq!(record.selling().asset_issuer().as_ref().unwrap(), SELLING_ASSET_ISSUER);
        assert_eq!(record.buying().asset_type(), BUYING_ASSET_TYPE);
        assert_eq!(record.buying().asset_code().as_ref().unwrap(), BUYING_ASSET_CODE);
        assert_eq!(record.buying().asset_issuer().as_ref().unwrap(), BUYING_ASSET_ISSUER);
        assert_eq!(record.amount(), AMOUNT);
        assert_eq!(record.price_ratio().numenator(), PRICE_R_N);
        assert_eq!(record.price_ratio().denominator(), PRICE_R_D);
        assert_eq!(record.price_decimal(), PRICE);
        assert_eq!(record.last_modified_ledger(), LAST_MODIFIED_LEDGER);
        assert_eq!(record.last_modified_time(), LAST_MODIFIED_TIME);
    }

    #[tokio::test]
    async fn test_get_offers_for_account() {
        const ACCOUNT_ID: &str = "GCXRNJ23TEHRNXQJEYXGQ3IYGVAWWY6Z2VOOWPP6STTYQCKXIRTNCN3E";
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org"
            .to_string())
            .unwrap();

        let offers_for_account_request =
            OffersForAccountRequest::new()
            .set_account_id(ACCOUNT_ID.to_string())
            .unwrap();

        let offers_for_account_response = horizon_client
            .get_offers_for_account(&offers_for_account_request)
            .await;
        
        assert!(offers_for_account_response.clone().is_ok());
        let binding = offers_for_account_response.unwrap();
        let record = &binding.embedded().records()[0];
        assert_eq!(record.links().self_link().href().as_ref().unwrap(), LINK_SELF);
        assert_eq!(record.links().offer_maker().href().as_ref().unwrap(), LINK_OFFER_MAKER);
        assert_eq!(record.id(), OFFER_ID);
        assert_eq!(record.paging_token(), PAGING_TOKEN);
        assert_eq!(record.seller(), SELLER);
        assert_eq!(record.selling().asset_type(), SELLING_ASSET_TYPE);
        assert_eq!(record.selling().asset_code().as_ref().unwrap(), SELLING_ASSET_CODE);
        assert_eq!(record.selling().asset_issuer().as_ref().unwrap(), SELLING_ASSET_ISSUER);
        assert_eq!(record.buying().asset_type(), BUYING_ASSET_TYPE);
        assert_eq!(record.buying().asset_code().as_ref().unwrap(), BUYING_ASSET_CODE);
        assert_eq!(record.buying().asset_issuer().as_ref().unwrap(), BUYING_ASSET_ISSUER);
        assert_eq!(record.amount(), AMOUNT);
        assert_eq!(record.price_ratio().numenator(), PRICE_R_N);
        assert_eq!(record.price_ratio().denominator(), PRICE_R_D);
        assert_eq!(record.price_decimal(), PRICE);
        assert_eq!(record.last_modified_ledger(), LAST_MODIFIED_LEDGER);
        assert_eq!(record.last_modified_time(), LAST_MODIFIED_TIME);
    }
}