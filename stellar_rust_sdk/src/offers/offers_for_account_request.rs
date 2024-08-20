use crate::models::*;
use stellar_rust_sdk_derive::Pagination;
use crate::Paginatable;

/// Represents the ID of an account for which the offers are to be retrieved.
#[derive(Default, Clone)]
pub struct OfferAccountId(String);

/// Represents the absence of the ID of an account for which the offers are to be retrieved.
#[derive(Default, Clone)]
pub struct NoOfferAccountId;
#[derive(Default)]

#[derive(Pagination)]
pub struct OffersForAccountRequest<I> {
    /// The ID of the account for which the offers are to be retrieved.
    account_id: I,
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

impl OffersForAccountRequest<NoOfferAccountId> {
    /// Creates a new `OffersForAccountRequest` with default parameters.
    pub fn new() -> Self {
        OffersForAccountRequest::default()
    }

    pub fn set_account_id(
        self,
        account_id: String,
    ) -> Result<OffersForAccountRequest<OfferAccountId>, String> {
        if let Err(e) = is_public_key(&account_id) {
            return Err(e.to_string());
        }

        Ok(OffersForAccountRequest {
            account_id: OfferAccountId(account_id,),
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        })
    }
}

impl Request for OffersForAccountRequest<OfferAccountId> {
    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        query.push_str(&format!("{}", self.account_id.0));

        query.trim_end_matches('&').to_string()
    }

    fn build_url(&self, base_url: &str) -> String {
        // This URL is not built with query paramaters, but with the account ID as addition to the path.
        // Therefore there is no `?` but a `/` in the formatted string.
        // Additionally, this request uses the API endpoint for `accounts`.
        use crate::accounts::ACCOUNTS_PATH;
        format!(
            "{}/{}/{}/{}",
            base_url,
            ACCOUNTS_PATH,
            self.get_query_parameters(),
            super::OFFERS_PATH
        )
    }
}