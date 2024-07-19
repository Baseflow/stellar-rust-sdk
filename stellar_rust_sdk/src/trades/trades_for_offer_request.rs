use crate::models::*;
use stellar_rust_sdk_derive::Pagination;
use crate::Paginatable;

/// Represents the ID of an offer for which the trades are to be retrieved.
#[derive(Default, Clone)]
pub struct TradeOfferId(String);

/// Represents the absence of an ID of an offer for which the trades are to be retrieved.
#[derive(Default, Clone)]
pub struct NoTradeOfferId;

#[derive(Default, Pagination)]
pub struct TradesForOfferRequest<I> {
    /// The ID of the offer for which the trades are to be retrieved.
    offer_id: I,
    /// A pointer to a specific location in a collection of responses, derived from the
    /// `paging_token` value of a record. Used for pagination control in the API response.
    pub cursor: Option<u32>,
    /// Specifies the maximum number of records to be returned in a single response.
    /// The range for this parameter is from 1 to 200. The default value is set to 10.
    pub limit: Option<u8>,
    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    /// and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    pub order: Option<Order>,
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