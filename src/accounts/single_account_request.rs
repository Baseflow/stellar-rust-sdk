use crate::models::{is_public_key, Request};

/// Represents a request to fetch details of a single account from the Horizon API.
///
/// `SingleAccountRequest` is a struct tailored to querying details of a specific account
/// on the Horizon API. This struct is designed to be used in conjunction with the 
/// [`HorizonClient::get_single_account`](crate::horizon_client::HorizonClient::get_single_account) method.
/// 
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
/// # use stellar_rust_sdk::accounts::prelude::SingleAccountRequest;
/// # use crate::stellar_rust_sdk::models::Request;
/// let mut request = SingleAccountRequest::new();
/// request.set_account_id("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string());
/// // Use with HorizonClient::get_single_account
/// ```
///
pub struct SingleAccountRequest {
    account_id: Option<String>,
}

impl Request for SingleAccountRequest {
    fn new() -> Self {
        SingleAccountRequest { account_id: None }
    }

    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        if let Some(account_id) = &self.account_id {
            query.push_str(&format!("{}", account_id));
        }

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

    fn validate(&self) -> Result<(), String> {
        if let Some(account_id) = &self.account_id {
            let is_valid = is_public_key(account_id);
            if is_valid.is_err() {
                return Err(is_valid.unwrap_err());
            }
        }

        Ok(())
    }
}

impl SingleAccountRequest {
    /// Sets the account ID for the request.
    ///
    /// # Arguments
    /// * `account_id` - A `String` specifying the account's public key.
    ///
    pub fn set_account_id(&mut self, account_id: String) -> &mut Self {
        self.account_id = Some(account_id);
        self
    }
}
