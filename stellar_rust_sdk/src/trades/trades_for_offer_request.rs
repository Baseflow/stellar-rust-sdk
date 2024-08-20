use crate::models::*;
use stellar_rust_sdk_derive::pagination;

/// Represents the ID of an offer for which the trades are to be retrieved.
#[derive(Default, Clone)]
pub struct TradeOfferId(String);

/// Represents the absence of an ID of an offer for which the trades are to be retrieved.
#[derive(Default, Clone)]
pub struct NoTradeOfferId;

#[pagination]
#[derive(Default)]
pub struct TradesForOfferRequest<I> {
    /// The ID of the offer for which the trades are to be retrieved.
    offer_id: I,
}

impl TradesForOfferRequest<TradeOfferId> {
    /// Creates a new `TradesForOfferRequest` with default parameters.
    pub fn new() -> Self {
        TradesForOfferRequest::default()
    }

    /// Sets the offer ID for the request.
    ///
    /// # Arguments
    /// * `offer_id` - The offer ID for which the trades are to be retrieved.
    ///
    /// # Returns
    /// A `TradesForOfferRequest` with the specified offer ID, or an error if the offer ID is invalid.
    ///
    pub fn set_offer_id(
        self,
        offer_id: String,
    ) -> Result<TradesForOfferRequest<TradeOfferId>, String> {
        Ok(TradesForOfferRequest {
            offer_id: TradeOfferId(offer_id),
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        })
    }
}

impl Request for TradesForOfferRequest<TradeOfferId> {
    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        query.push_str(&format!("{}", self.offer_id.0));

        query.trim_end_matches('&').to_string()
    }

    fn build_url(&self, base_url: &str) -> String {
        // This URL is not built with query paramaters, but with the offer's ID as addition to the path.
        // Therefore there is no `?` but a `/` in the formatted string.
        // Additionally, this request uses the API endpoint for `offers`.
        use crate::offers::OFFERS_PATH;
        format!(
            "{}/{}/{}/{}",
            base_url,
            OFFERS_PATH,
            self.get_query_parameters(),
            super::TRADES_PATH
        )
    }
}