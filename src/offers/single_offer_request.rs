use crate::models::*;

/// Represents the offer ID.
#[derive(Default, Clone)]
pub struct OfferId(String);

/// Represents the absence of an offer ID.
#[derive(Default, Clone)]
pub struct NoOfferId;

/// Represents a request to fetch details of an offer from the Horizon API.
///
/// `SingleOfferRequest` is a struct tailored to querying details of a specific offer
/// on the Horizon API. This struct is designed to be used in conjunction with the
/// [`HorizonClient::get_single_offer`](crate::horizon_client::HorizonClient::get_single_offer) method.
///
/// The struct matches the parameters necessary to construct a request for the
/// <a href="https://developers.stellar.org/network/horizon/resources/get-offer-by-offer-id">Retrieve An Offer endpoint</a>
/// of the Horizon API.
///
/// # Fields
/// Required:
/// * `offer_id` - The offer's ID.
///
/// ## Usage
/// Instances of `SingleOfferRequest` are created and configured using setter methods for each
/// parameter.
/// ```
/// # use stellar_rs::offers::prelude::SingleOfferRequest;
/// # use stellar_rs::models::Request;
/// let request = SingleOfferRequest::new()
///     .set_offer_id("1".to_string()); // example offer ID
///
/// // Use with HorizonClient::get_single_offer
/// ```
///
#[derive(Default)]
pub struct SingleOfferRequest<I> {
    /// The ID of the offer to be retrieved.
    offer_id: I,
}

impl SingleOfferRequest<NoOfferId> {
    /// Creates a new `SingleOfferRequest` with default parameters.
    pub fn new() -> Self {
        SingleOfferRequest::default()
    }

    /// Sets the offer ID for the request.
    ///
    /// # Arguments
    /// * `offer_id` - The offer ID to retrieve.
    ///
    /// # Returns
    /// A `SingleOfferRequest` with the specified offer ID, or an error if the offer ID is invalid.
    ///
    pub fn set_offer_id(
        self,
        offer_id: String,
    ) -> Result<SingleOfferRequest<OfferId>, String> {
        match offer_id.parse::<u32>() {
            Ok(id) => {
                if id > 0 {
                    Ok(SingleOfferRequest {
                        offer_id: OfferId(offer_id)
                    })
                } else {
                    Err("offer ID must be greater than or equal to 1".to_string())
                }
            }
            Err(_) => Err("invalid offer ID".to_string()),
        }
    }
}

impl Request for SingleOfferRequest<OfferId> {
    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        query.push_str(&format!("{}", self.offer_id.0));

        query.trim_end_matches('&').to_string()
    }

    fn build_url(&self, base_url: &str) -> String {
        // This URL is not built with query paramaters, but with the offer ID as addition to the path.
        // Therefore there is no `?` but a `/` in the formatted string.
        format!(
            "{}/{}/{}",
            base_url,
            super::OFFERS_PATH,
            self.get_query_parameters()
        )
    }
}