use crate::models::*;
use stellar_rust_sdk_derive::pagination;

/// Represents the ID of an account for which the trades are to be retrieved.
#[derive(Default, Clone)]
pub struct TradeAccountId(String);

/// Represents the absence of the ID of an account for which the trades are to be retrieved.
#[derive(Default, Clone)]
pub struct NoTradeAccountId;

#[pagination]
#[derive(Default)]
pub struct TradesForAccountRequest<I> {
    /// The ID of the account for which the trades are to be retrieved.
    account_id: I,
}

impl TradesForAccountRequest<NoTradeAccountId> {
    /// Creates a new `TradesForAccountRequest` with default parameters.
    pub fn new() -> Self {
        TradesForAccountRequest::default()
    }

    /// Sets the account ID for the request.
    ///
    /// # Arguments
    /// * `account_id` - The account ID for which the trades are to be retrieved.
    ///
    /// # Returns
    /// A `TradesForAccountRequest` with the specified account ID, or an error if the account ID is invalid.
    ///
    pub fn set_account_id(
        self,
        account_id: String,
    ) -> Result<TradesForAccountRequest<TradeAccountId>, String> {
        if let Err(e) = is_public_key(&account_id) {
            return Err(e.to_string());
        }

        Ok(TradesForAccountRequest {
            account_id: TradeAccountId(account_id),
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        })
    }
}

impl Request for TradesForAccountRequest<TradeAccountId> {
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
            super::TRADES_PATH
        )
    }
}
