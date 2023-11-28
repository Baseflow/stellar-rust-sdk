use crate::models::{is_public_key, Request};

#[derive(Default, Clone)]
pub struct AccountId(String);
#[derive(Default, Clone)]
pub struct NoAccountId;

/// SingleAccountRequest is the request for the /accounts endpoint to get a single account.
/// [More Details](https://developers.stellar.org/api/horizon/resources/retrieve-an-account "Accounts")
#[derive(Default)]
pub struct SingleAccountRequest<I> {
    /// Account ID of the sponsor. Every account in the response will either be sponsored by the given account ID or have a subentry (trustline, offer, or data entry) which is sponsored by the given account ID.
    account_id: I,
}

impl SingleAccountRequest<NoAccountId> {
    pub fn new() -> Self {
        SingleAccountRequest::default()
    }
}

impl<I> SingleAccountRequest<I> {
    /// Sets the account ID of the account to get.
    /// # Arguments
    /// * `account_id` - The account ID of the account to get.
    /// # Returns
    /// The request object
    /// [SingleAccountRequest](struct.SingleAccountRequest.html)
    pub fn set_account_id(self, account_id: String) -> Result<SingleAccountRequest<AccountId>, String> {
        if let Err(e) = is_public_key(&account_id) {
            return Err(e.to_string());
        }

        Ok(SingleAccountRequest {
            account_id: AccountId(account_id)
        })
    }
}

impl Request for SingleAccountRequest<AccountId> {
    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        query.push_str(&format!("{}", self.account_id.0));

        query.trim_end_matches('&').to_string()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}/{}",
            base_url,
            super::ACCOUNTS_PATH,
            self.get_query_parameters()
        )
    }
}
