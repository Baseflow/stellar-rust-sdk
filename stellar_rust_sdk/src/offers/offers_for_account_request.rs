use crate::models::*;

#[derive(Default)]
pub struct OffersForAccountRequest {
    /// The ID of the account for which the offers are to be retrieved.
    account_id: String,
}

impl OffersForAccountRequest {
    /// Creates a new `OffersForAccountRequest` with default parameters.
    pub fn new() -> Self {
        OffersForAccountRequest::default()
    }

    pub fn set_account_id(
        self,
        account_id: String,
    ) -> Result<OffersForAccountRequest, String> {
        if let Err(e) = is_public_key(&account_id) {
            return Err(e.to_string());
        }

        Ok(OffersForAccountRequest {
            account_id: account_id,
        })
    }
}

impl Request for OffersForAccountRequest {
    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        query.push_str(&format!("{}", self.account_id));

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
