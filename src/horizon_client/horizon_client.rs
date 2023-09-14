use crate::{models::Request, assets::prelude::{AllAssetsRequest, AllAssetsResponse}};
use reqwest;
use url::Url;

use crate::accounts::prelude::*;

pub struct HorizonClient {
    /// The base URL for the Horizon server
    base_url: String,
}

impl HorizonClient {
    /// Creates a new Horizon client
    /// # Arguments
    /// * `base_url` - The base URL for the Horizon server
    /// # Returns
    /// The Horizon client
    pub fn new(base_url: String) -> Result<Self, String> {
        url_validate(&base_url)?;
        Ok(Self { base_url })
    }

    /// Gets the base URL for the Horizon server
    /// # Arguments
    /// * `self` - The Horizon client
    /// * request - The accounts request
    /// # Returns
    /// The accounts response
    /// # Errors
    /// Returns an error if the request fails
    /// [GET /accounts](https://www.stellar.org/developers/horizon/reference/endpoints/accounts.html)
    pub async fn get_account_list(
        &self,
        request: &AccountsRequest,
    ) -> Result<AccountsResponse, String> {
        self.get::<AccountsResponse>(request).await
    }

    /// Gets the base URL for the Horizon server
    /// # Arguments
    /// * `self` - The Horizon client
    /// * request - The account request
    /// # Returns
    /// The account response
    /// # Errors
    /// Returns an error if the request fails
    /// [GET /accounts/{account_id}](https://www.stellar.org/developers/horizon/reference/endpoints/accounts-single.html)
    pub async fn get_single_account(
        &self,
        request: &SingleAccountRequest,
    ) -> Result<SingleAccountsResponse, String> {
        self.get::<SingleAccountsResponse>(request).await
    }

    pub async fn get_all_assets(
        &self,
        request: &AllAssetsRequest,
    ) -> Result<AllAssetsResponse, String> {
        self.get::<AllAssetsResponse>(request).await
    }

    /// Sends a GET request to the server
    /// # Arguments
    /// * `TResponse` - The type of the response
    /// # Returns
    /// The response from the server
    /// # Errors
    /// Returns an error if the request fails
    async fn get<TResponse: Default>(&self, request: &impl Request) -> Result<TResponse, String> {
        // Validate the request.
        request.validate()?;

        //match request by SingleAccountRequest or AccountsRequest
        // Determine the url
        // TODO: construct with query parameters
        let url = request.build_url(&self.base_url);
        let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
        let result: TResponse = handle_response(response).await?;
        Ok(result)
    }
}

/// Handles the response from the server
/// # Arguments
/// * `response` - The response from the server
/// # Returns
/// The deserialized response from the server response payload
/// # Errors
/// Returns an error if the response is not successful
async fn handle_response<TResponse: Default>(
    response: reqwest::Response,
) -> Result<TResponse, String> {
    println!("Response: {:?}", response);
    match response.status() {
        reqwest::StatusCode::OK => {
            let _response = response.text().await.map_err(|e| e.to_string())?;
            Ok(TResponse::default())
            //decode(&response.as_bytes()).map_err(|e| e.to_string());
            // match result {
            //     Ok(response) => Ok(response),
            //     Err(error) => Err(error),
            // }
        }
        _ => {
            let response = response.text().await.map_err(|e| e.to_string())?;
            Err(response)
        }
    }
}
/// url_validate validates a URL
fn url_validate(url: &str) -> Result<(), String> {
    // check if start with http:// or https://
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(format!("URL must start with http:// or https://: {}", url));
    }
    Url::parse(url).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::assets::prelude::AllAssetsRequest;

    use super::*;

    #[test]
    fn test_url_validate_invalid_url() {
        let result = url_validate("horizon-testnet.stellar.org");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "URL must start with http:// or https://: horizon-testnet.stellar.org"
        );
    }

    #[test]
    fn test_url_validate_valid_url() {
        let result = url_validate("https://horizon-testnet.stellar.org");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_account_list() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let mut accounts_request = AccountsRequest::new();
        accounts_request
            .set_sponsor("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7")
            .set_limit(10);

        // call the get_account_list method to retrieve the account list response
        let _accounts_response = horizon_client.get_account_list(&accounts_request).await;
        // will throw exception for now
        assert!(_accounts_response.is_ok());

        // assert_eq!(accounts_response.accounts.len(), 0);
    }

    #[tokio::test]
    async fn test_get_single_account() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let mut single_account_request = SingleAccountRequest::new();
        single_account_request
            .set_account_id("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string());

        let _single_account_response = horizon_client
            .get_single_account(&single_account_request)
            .await;

        assert!(_single_account_response.is_ok());
    }

    #[tokio::test]
    async fn test_get_all_assests() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();
        
        // construct request
        let mut all_assets_request = AllAssetsRequest::new();
        all_assets_request.set_asset_issuer("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string());

        let _all_assets_response = horizon_client
            .get_all_assets(&all_assets_request)
            .await;

        assert!(_all_assets_response.is_ok());
    }
}
