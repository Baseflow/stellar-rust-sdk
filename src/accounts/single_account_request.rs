use crate::models::{is_public_key, Request};

/// Represents a query parameter for the account's public key
#[derive(Default, Clone)]
pub struct AccountId(String);
/// Represents the absence of a query parameter for the account's public key
#[derive(Default, Clone)]
pub struct NoAccountId;

/// Represents a request to fetch details of a single account from the Horizon API.
///
/// `SingleAccountRequest` is a struct tailored to querying details of a specific account
/// on the Horizon API. This struct is designed to be used in conjunction with the
/// [`HorizonClient::get_single_account`](crate::horizon_client::HorizonClient::get_single_account) method.
///
/// The struct matches the parameters necessary to construct a request for the
/// <a href="https://developers.stellar.org/api/horizon/resources/retrieve-an-account">Retrieve An Account endpoint</a>
/// of the Horizon API.
///
/// # Fields
/// Required:
/// * `account_id` - The account's public key.
///
/// ## Usage
/// Instances of `SingleAccountRequest` are created and configured using setter methods for each
/// parameter.
/// ```
/// # use stellar_rs::accounts::prelude::SingleAccountRequest;
/// # use stellar_rs::models::Request;
/// let request = SingleAccountRequest::new()
///     .set_account_id("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string())
///     .unwrap();
/// // Use with HorizonClient::get_single_account
/// ```
///
#[derive(Default)]
pub struct SingleAccountRequest<I> {
    /// The account's public key.
    account_id: I,
}

impl SingleAccountRequest<NoAccountId> {
    /// Creates a new `SingleAccountRequest` with default parameters.
    pub fn new() -> Self {
        SingleAccountRequest::default()
    }

    /// Sets the account ID for the request.
    ///
    /// # Arguments
    /// * `account_id` - A `String` specifying the account's public key.
    ///
    pub fn set_account_id(
        self,
        account_id: String,
    ) -> Result<SingleAccountRequest<AccountId>, String> {
        if let Err(e) = is_public_key(&account_id) {
            return Err(e.to_string());
        }

        Ok(SingleAccountRequest {
            account_id: AccountId(account_id),
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
        // This URL is not built with query paramaters, but with the AccountID as addition to the path.
        // therefore there is no `?` but a `/` in the formatted string.
        format!(
            "{}/{}/{}",
            base_url,
            super::ACCOUNTS_PATH,
            self.get_query_parameters()
        )
    }
}
