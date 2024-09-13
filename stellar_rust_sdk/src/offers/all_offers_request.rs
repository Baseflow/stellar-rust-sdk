use crate::{models::prelude::*, models::*, BuildQueryParametersExt};
use stellar_rust_sdk_derive::pagination;

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
/// use stellar_rs::models::{Order, prelude::*};
///
/// let request = AllOffersRequest::new()
///     .set_sponsor("GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5").unwrap() // Optional buyer filter
///     .set_seller("GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4").unwrap() // optional seller filter
///     .set_selling(IssuedOrNative::Issued(AssetData {
///         asset_code: "USDC".to_string(),
///         asset_issuer: "GBAKINTNEGR7PO6Z6XW2S5ITT5VARNW6DZ5K4OYSLFNEA2CSMUM2UEF4".to_string(),
///     })) // optional selling asset filter
///     .set_buying(IssuedOrNative::Native) // optional buying asset filter
///     .set_cursor(123).unwrap() // optional cursor for pagination
///     .set_limit(100).unwrap() // optional limit for response records
///     .set_order(Order::Desc).unwrap(); // optional order of records
///
/// // Use with HorizonClient::get_all_offers
/// ```
///
#[pagination]
#[derive(Default)]
pub struct AllOffersRequest {
    /// Optional. The ID of the sponsor. When set, the response will
    /// only include offers sponsored by the specified account.
    sponsor: Option<String>,
    /// Optional. The ID of the seller making the offer. When set, the response will
    /// only include offers created by by the specified account.
    seller: Option<String>,
    /// Optional. Indicates an selling asset for which offers are being queried.
    /// When set, the response will filter the offers that hold this specific asset.
    selling: Option<IssuedOrNative>,
    /// Optional. Indicates a buying asset for which offers are being queried.
    /// When set, the response will filter the offers that hold this specific asset.
    buying: Option<IssuedOrNative>,
}

impl Request for AllOffersRequest {
    fn get_query_parameters(&self) -> String {
        // Determine selling assets and form parameters.
        let prefix = "selling=";
        let selling_asset = match &self.selling {
            Some(IssuedOrNative::Native) => format!("{}native", prefix),
            Some(IssuedOrNative::Issued(asset_data)) => {
                format!(
                    "{}{}%3A{}",
                    prefix, asset_data.asset_code, asset_data.asset_issuer
                )
            },
            None => String::new()
        };

        // Determine buying assets and form parameters.
        let prefix = "buying=";
        let buying_asset = match &self.buying {
            Some(IssuedOrNative::Native) => format!("{}native", prefix),
            Some(IssuedOrNative::Issued(asset_data)) => {
                format!(
                    "{}{}%3A{}",
                    prefix, asset_data.asset_code, asset_data.asset_issuer
                )
            },
            None => String::new()
        };

        vec![
            self.sponsor.as_ref().map(|s| format!("sponsor={}", s)),
            self.seller.as_ref().map(|s| format!("seller={}", s)),
            Some(selling_asset),
            Some(buying_asset),
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
    pub fn set_sponsor(self, sponsor: impl Into<String>) -> Result<AllOffersRequest, String> {
        let sponsor = sponsor.into();
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
    pub fn set_seller(self, seller: impl Into<String>) -> Result<AllOffersRequest, String> {
        let seller = seller.into();
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
    pub fn set_selling(self, selling: IssuedOrNative) -> AllOffersRequest {
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
    pub fn set_buying(self, buying: IssuedOrNative) -> AllOffersRequest {
        AllOffersRequest {
            buying: Some(buying),
            ..self
        }
    }
}
