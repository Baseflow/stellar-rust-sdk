use crate::{models::*, BuildQueryParametersExt};
use stellar_rust_sdk_derive::Pagination;
use crate::Paginatable;

/// Represents a request to list all offers from the Stellar Horizon API.
///
/// This structure is used to construct a query to retrieve a comprehensive list of offers, which
/// can be filtered by sponsor, seller, selling asset and buying asset. It adheres to the structure and parameters required
/// by the Horizon API for retrieving a
/// <a href="https://developers.stellar.org/network/horizon/api-reference/resources/get-all-offers">list of all offers</a>.
///
/// # Usage
///
/// Create an instance of this struct and set the desired query parameters to filter the list of offers.
/// Pass this request object to the [`HorizonClient::get_all_offers`](crate::horizon_client::HorizonClient::get_all_offers)
/// method to fetch the corresponding data from the Horizon API.
///
/// # Example
/// ```
/// use stellar_rs::offers::all_offers_request::AllOffersRequest;
/// use stellar_rs::models::{Asset, NativeAsset, Order};
/// use stellar_rust_sdk_derive::Pagination;
/// use stellar_rs::Paginatable;
///
/// let request = AllOffersRequest::new()
///     .set_sponsor("GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5".to_string()).unwrap() // Optional sponsor filter
///     .set_seller("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string()).unwrap() // Optional seller filter
///     .set_selling(Asset::new().set_issued("USD", "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7").unwrap()) // Optional selling asset filter
///     .set_buying(Asset::new().set_issued("USD", "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7").unwrap()) // Optional buying asset filter
///     .set_cursor(123).unwrap() // Optional cursor for pagination
///     .set_limit(100).unwrap() // Optional limit for response records
///     .set_order(Order::Desc); // Optional order of records
///
/// // Use with HorizonClient::get_all_offers
/// ```
///
#[derive(Default, Pagination)]
pub struct AllOffersRequest {
    /// Optional. The ID of the sponsor. When set, the response will
    /// only include offers sponsored by the specified account.
    sponsor: Option<String>,
    /// Optional. The ID of the seller making the offer. When set, the response will
    /// only include offers created by by the specified account.
    seller: Option<String>,
    /// Optional. Indicates an selling asset for which offers are being queried.
    /// When set, the response will filter the offers that hold this specific asset.
    selling: Option<Asset<IssuedAsset>>,
    /// Optional. Indicates a buying asset for which offers are being queried.
    /// When set, the response will filter the offers that hold this specific asset.
    buying: Option<Asset<IssuedAsset>>,
    /// A pointer to a specific location in a collection of responses, derived from the
    /// `paging_token` value of a record. Used for pagination control in the API response.
    cursor: Option<u32>,
    /// Specifies the maximum number of records to be returned in a single response.
    /// The range for this parameter is from 1 to 200. The default value is set to 10.
    limit: Option<u8>,
    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    /// and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    order: Option<Order>,
}

impl Request for AllOffersRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.sponsor.as_ref().map(|s| format!("sponsor={}", s)),
            self.seller.as_ref().map(|s| format!("seller={}", s)),
            self.selling.as_ref().map(|s| format!("selling={}", s)),
            self.buying.as_ref().map(|b| format!("buying={}", b)),
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}/{}",
            base_url,
            super::OFFERS_PATH,
            self.get_query_parameters()
        )
    }
}

impl AllOffersRequest {
    /// Creates a new `AllOffersRequest` with default parameters.
    pub fn new() -> Self {
        AllOffersRequest::default()
    }

    /// Specifies the sponsor's public key in the request.
    ///
    /// # Arguments
    /// * `sponsor` - A Stellar public key of the sponsor to filter offers by.
    ///
    pub fn set_sponsor(self, sponsor: String) -> Result<AllOffersRequest, String> {
        if let Err(e) = is_public_key(&sponsor) {
            return Err(e.to_string());
        }

        Ok(AllOffersRequest {
            sponsor: Some(sponsor),
            ..self
        })
    }

    /// Specifies the seller's public key in the request.
    ///
    /// # Arguments
    /// * `seller` - A Stellar public key of the seller to filter offers by.
    ///
    pub fn set_seller(self, seller: String) -> Result<AllOffersRequest, String> {
        if let Err(e) = is_public_key(&seller) {
            return Err(e.to_string());
        }

        Ok(AllOffersRequest {
            seller: Some(seller),
            ..self
        })
    }

    /// Specifies the selling asset in the request.
    ///
    /// # Arguments
    /// * `selling` - The selling asset to filter offers by.
    ///
    pub fn set_selling(self, selling: Asset<IssuedAsset>) -> AllOffersRequest {
        AllOffersRequest {
            selling: Some(selling),
            ..self
        }
    }

    /// Specifies the buying asset in the request.
    ///
    /// # Arguments
    /// * `buying` - The buying asset to filter offers by.
    ///
    pub fn set_buying(self, buying: Asset<IssuedAsset>) -> AllOffersRequest {
        AllOffersRequest {
            buying: Some(buying),
            ..self
        }
    }
}