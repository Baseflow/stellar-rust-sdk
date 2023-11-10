use crate::{
    accounts::prelude::{
        AccountsRequest, AccountsResponse, SingleAccountRequest, SingleAccountsResponse,
    },
    assets::prelude::{AllAssetsRequest, AllAssetsResponse},
    claimable_balances::prelude::{AllClaimableBalancesRequest, AllClaimableBalancesResponse},
    ledgers::prelude::{
        LedgersRequest, LedgersResponse, SingleLedgerRequest, SingleLedgerResponse,
    },
    models::{Request, Response},
};
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

    /// Gets the base URL for the Horizon server
    /// # Arguments
    /// * `self` - The Horizon client
    /// * request - The all assets request
    /// # Returns
    /// The all assets response
    /// # Errors
    /// Returns an error if the request fails
    /// [GET /assets](https://www.stellar.org/developers/horizon/reference/endpoints/assets-all.html)
    pub async fn get_all_assets(
        &self,
        request: &AllAssetsRequest,
    ) -> Result<AllAssetsResponse, String> {
        self.get::<AllAssetsResponse>(request).await
    }

    /// Gets the base URL for the Horizon server
    /// # Arguments
    /// * `self` - The Horizon client
    /// * request - The all claimable balances request
    /// # Returns
    /// The all claimable balances response
    /// # Errors
    /// Returns an error if the request fails
    /// [GET /claimable_balances/all](https://www.stellar.org/developers/horizon/reference/endpoints/claimable_balances-all.html)
    pub async fn get_all_claimable_balances(
        &self,
        request: &AllClaimableBalancesRequest,
    ) -> Result<AllClaimableBalancesResponse, String> {
        self.get::<AllClaimableBalancesResponse>(request).await
    }

    pub async fn get_all_ledgers(
        &self,
        request: &LedgersRequest,
    ) -> Result<LedgersResponse, String> {
        self.get::<LedgersResponse>(request).await
    }

    pub async fn get_single_ledger(
        &self,
        request: &SingleLedgerRequest,
    ) -> Result<SingleLedgerResponse, String> {
        self.get::<SingleLedgerResponse>(request).await
    }

    /// Sends a GET request to the server
    /// # Arguments
    /// * `TResponse` - The type of the response
    /// # Returns
    /// The response from the server
    /// # Errors
    /// Returns an error if the request fails
    async fn get<TResponse: Response + std::fmt::Debug>(
        &self,
        request: &impl Request,
    ) -> Result<TResponse, String> {
        // Validate the request.
        request.validate()?;

        //match request by SingleAccountRequest or AccountsRequest
        // Determine the url
        // TODO: construct with query parameters

        let url = request.build_url(&self.base_url);
        println!("\n\nURL: {}", url);
        let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
        let result: TResponse = handle_response(response).await?;

        // print!("\n\nResult: {:?}", result);
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
async fn handle_response<TResponse: Response>(
    response: reqwest::Response,
) -> Result<TResponse, String> {
    // println!("\n Response: {:?}", response);
    match response.status() {
        reqwest::StatusCode::OK => {
            let _response = response.text().await.map_err(|e| e.to_string())?;
            // println!("\n\nResponse: {:?}", _response);
            TResponse::from_json(_response)
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
    use base64::encode;
    use chrono::{DateTime, TimeZone, Utc};

    use crate::{assets::prelude::AllAssetsRequest, ledgers::prelude::SingleLedgerRequest};

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
            .set_signer("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7")
            .set_limit(10);

        // call the get_account_list method to retrieve the account list response
        let _accounts_response: Result<AccountsResponse, String> =
            horizon_client.get_account_list(&accounts_request).await;

        assert!(_accounts_response.is_ok());

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_account_id(),
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string()
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_id(),
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string()
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_sequence(),
            "4380492979765248".to_string()
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_subentry_count(),
            0
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_last_modified_ledger(),
            1019913
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_last_modified_time(),
            "2023-08-15T09:46:25Z"
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_thresholds()
                .get_low_threshold(),
            0
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_thresholds()
                .get_med_threshold(),
            0
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_thresholds()
                .get_high_threshold(),
            0
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_flags()
                .get_auth_required(),
            false
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_flags()
                .get_auth_revocable(),
            false
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_flags()
                .get_auth_immutable(),
            false
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_flags()
                .get_auth_clawback_enabled(),
            false
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_balances()[0]
                .get_balance(),
            "10000.0000000".to_string()
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_balances()[0]
                .get_asset_type(),
            "native".to_string()
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_balances()[0]
                .get_buying_liabilities(),
            "0.0000000".to_string()
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_balances()[0]
                .get_selling_liabilities(),
            "0.0000000".to_string()
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_signers()[0]
                .get_key(),
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string()
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_signers()[0]
                .get_weight(),
            1
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_signers()[0]
                .get_type(),
            "ed25519_public_key".to_string()
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_num_sponsoring(),
            0
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_num_sponsored(),
            0
        );

        assert_eq!(
            _accounts_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_paging_token(),
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string()
        );
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

        assert_eq!(
            _single_account_response
                .clone()
                .unwrap()
                .get_account_id()
                .to_string(),
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string()
        );

        assert_eq!(
            _single_account_response
                .clone()
                .unwrap()
                .get_sequence()
                .to_string(),
            "4380492979765248".to_string()
        );

        assert_eq!(
            _single_account_response
                .clone()
                .as_ref()
                .unwrap()
                .get_subentry_count(),
            0
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .get_last_modified_ledger(),
            1019913
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .get_last_modified_time(),
            "2023-08-15T09:46:25Z"
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .get_thresholds()
                .get_low_threshold(),
            0
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .get_thresholds()
                .get_med_threshold(),
            0
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .get_thresholds()
                .get_high_threshold(),
            0
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .get_flags()
                .get_auth_required(),
            false
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .get_flags()
                .get_auth_revocable(),
            false
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .get_flags()
                .get_auth_immutable(),
            false
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .get_flags()
                .get_auth_clawback_enabled(),
            false
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().get_balances()[0].get_balance(),
            "10000.0000000".to_string()
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().get_balances()[0].get_asset_type(),
            "native".to_string()
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().get_balances()[0].get_buying_liabilities(),
            "0.0000000".to_string()
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().get_balances()[0].get_selling_liabilities(),
            "0.0000000".to_string()
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().get_signers()[0].get_key(),
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string()
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().get_signers()[0].get_weight(),
            1
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().get_signers()[0].get_type(),
            "ed25519_public_key".to_string()
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .get_num_sponsoring(),
            0
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .get_num_sponsored(),
            0
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .get_paging_token(),
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string()
        );
    }

    #[tokio::test]
    async fn test_get_all_assests() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let mut all_assets_request: AllAssetsRequest = AllAssetsRequest::new();
        all_assets_request
            // .set_asset_issuer("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7")
            .set_limit(1);

        let _all_assets_response = horizon_client.get_all_assets(&all_assets_request).await;

        assert!(_all_assets_response.is_ok());

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_asset_type(),
            "credit_alphanum4".to_string()
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_asset_code(),
            "0".to_string()
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_asset_issuer(),
            "GCINFW5NLMVSE7KWH5BOVL2NTRP2HN6LSXTIL76GOVIORLFM6YN5ZTRS"
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_paging_token(),
            "0_GCINFW5NLMVSE7KWH5BOVL2NTRP2HN6LSXTIL76GOVIORLFM6YN5ZTRS_credit_alphanum4"
                .to_string()
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_num_accounts(),
            0
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_num_claimable_balances(),
            0
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_num_liquidity_pools(),
            0
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_amount(),
            "0.0000000".to_string()
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_accounts()
                .get_authorized(),
            0
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_accounts()
                .get_authorized_to_maintain_liabilities(),
            0
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_accounts()
                .get_unauthorized(),
            1
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_claimable_balances_amount(),
            "0.0000000".to_string()
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_liquidity_pools_amount(),
            "0.0000000".to_string()
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_contracts_amount(),
            "0.0000000".to_string()
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_balances()
                .get_authorized(),
            "0.0000000".to_string()
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_balances()
                .get_authorized_to_maintain_liabilities(),
            "0.0000000".to_string()
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_balances()
                .get_unauthorized(),
            "1.0000000".to_string()
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_flags()
                .get_auth_required(),
            true
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_flags()
                .get_auth_revocable(),
            true
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_flags()
                .get_auth_immutable(),
            false
        );

        assert_eq!(
            _all_assets_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_records()[0]
                .get_flags()
                .get_auth_clawback_enabled(),
            true
        );
    }

    #[tokio::test]
    async fn test_get_all_ledgers() {
        // Initialize horizon client

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let mut all_ledgers_request = LedgersRequest::new();
        all_ledgers_request.set_limit(2);

        let _all_ledgers_response = horizon_client.get_all_ledgers(&all_ledgers_request).await;

        assert!(_all_ledgers_response.clone().is_ok());

        assert_eq!(
            _all_ledgers_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_single_record(0)
                .get_hash(),
            "eca856e0073dc2087249dc929ed31c09c3babfef2e687b685d0513dbe6489a18"
        );

        assert_eq!(
            _all_ledgers_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_single_record(0)
                .get_prev_hash(),
            "63d98f536ee68d1b27b5b89f23af5311b7569a24faf1403ad0b52b633b07be99".to_string()
        );

        assert_eq!(
            _all_ledgers_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_single_record(0)
                .get_sequence(),
            2
        );

        assert_eq!(
            _all_ledgers_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_single_record(0)
                .get_successful_transaction_count(),
            0
        );

        assert_eq!(
            _all_ledgers_response
                .clone()
                .unwrap()
                .get__embedded()
                .get_single_record(0)
                .get_paging_token(),
            "8589934592".to_string()
        );
    }

    #[tokio::test]
    async fn test_get_single_ledger() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let mut single_ledger_request = SingleLedgerRequest::new();
        single_ledger_request.set_sequence(2);

        let _single_ledger_response = horizon_client
            .get_single_ledger(&single_ledger_request)
            .await;

        assert!(_single_ledger_response.clone().is_ok());

        assert_eq!(
            _single_ledger_response.clone().unwrap().get_id(),
            "eca856e0073dc2087249dc929ed31c09c3babfef2e687b685d0513dbe6489a18".to_string()
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().get_paging_token(),
            "8589934592".to_string()
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().get_hash(),
            "eca856e0073dc2087249dc929ed31c09c3babfef2e687b685d0513dbe6489a18".to_string()
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().get_prev_hash(),
            "63d98f536ee68d1b27b5b89f23af5311b7569a24faf1403ad0b52b633b07be99".to_string()
        );

        assert_eq!(_single_ledger_response.clone().unwrap().get_sequence(), 2);

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_successful_transaction_count(),
            0
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_failed_transaction_count(),
            0
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_operation_count(),
            0
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_tx_set_operation_count(),
            0
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().get_closed_at(),
            "2023-06-14T09:19:48Z".to_string()
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().get_total_coins(),
            "100000000000.0000000"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().get_fee_pool(),
            "0.0000000"
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_base_fee_in_stroops(),
            100
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_base_reserve_in_stroops(),
            100000000
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_max_tx_set_size(),
            100
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_protocol_version(),
            0
        );

        let decoded_xdr_header = _single_ledger_response
            .unwrap()
            .get_decoded_header_xdr()
            .unwrap();

        assert_eq!(
            decoded_xdr_header.bucket_list_hash.to_string(),
            "735227ed398461291237687b08446aa2c9b096e0c98a462dadda569f05dd2484"
        );

        assert_eq!(decoded_xdr_header.ledger_seq, 2);

        assert_eq!(decoded_xdr_header.total_coins, 1000000000000000000);

        assert_eq!(decoded_xdr_header.fee_pool, 0);

        assert_eq!(decoded_xdr_header.inflation_seq, 0);

        assert_eq!(decoded_xdr_header.id_pool, 0);

        assert_eq!(decoded_xdr_header.base_fee, 100);

        assert_eq!(decoded_xdr_header.base_reserve, 100000000);

        assert_eq!(decoded_xdr_header.max_tx_set_size, 100);

        let tx_set_hash = decoded_xdr_header.scp_value.tx_set_hash.to_string();
        let tx_set_hash_bytes = hex::decode(tx_set_hash).expect("Failed to decode hex");
        let tx_set_hash_base64 = encode(&tx_set_hash_bytes);

        assert_eq!(
            tx_set_hash_base64,
            "uZRHr9UdXKbTKiclfOjy72YZFJUkJPVcKT5htvorm1Q="
        );

        assert_eq!(
            decoded_xdr_header.scp_value.close_time,
            stellar_xdr::TimePoint(1686734388)
        );
    }

    #[tokio::test]
    async fn test_get_decoded_single_ledger() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let mut single_ledger_request = SingleLedgerRequest::new();
        single_ledger_request.set_sequence(2);

        let _single_ledger_response = horizon_client
            .get_single_ledger(&single_ledger_request)
            .await;

        assert!(_single_ledger_response.is_ok());

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_id()
                .to_string(),
            "eca856e0073dc2087249dc929ed31c09c3babfef2e687b685d0513dbe6489a18"
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_paging_token()
                .to_string(),
            "8589934592"
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_hash()
                .to_string(),
            "eca856e0073dc2087249dc929ed31c09c3babfef2e687b685d0513dbe6489a18"
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_prev_hash()
                .to_string(),
            "63d98f536ee68d1b27b5b89f23af5311b7569a24faf1403ad0b52b633b07be99"
        );

        assert_eq!(_single_ledger_response.clone().unwrap().get_sequence(), 2);

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_successful_transaction_count(),
            0
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_failed_transaction_count(),
            0
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_operation_count(),
            0
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_tx_set_operation_count(),
            0
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_closed_at()
                .to_string(),
            "2023-06-14T09:19:48Z"
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_total_coins()
                .to_string(),
            "100000000000.0000000"
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_fee_pool()
                .to_string(),
            "0.0000000"
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_base_fee_in_stroops(),
            100
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_base_reserve_in_stroops(),
            100000000
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_max_tx_set_size(),
            100
        );

        assert_eq!(
            _single_ledger_response
                .clone()
                .unwrap()
                .get_protocol_version(),
            0
        );

        let _decoded_header_xdr = _single_ledger_response
            .unwrap()
            .get_decoded_header_xdr()
            .unwrap();

        assert_eq!(
            _decoded_header_xdr.bucket_list_hash.to_string(),
            "735227ed398461291237687b08446aa2c9b096e0c98a462dadda569f05dd2484"
        );

        assert_eq!(_decoded_header_xdr.ledger_seq, 2);
        assert_eq!(_decoded_header_xdr.total_coins, 1000000000000000000);
        assert_eq!(_decoded_header_xdr.fee_pool, 0);
        assert_eq!(_decoded_header_xdr.inflation_seq, 0);
        assert_eq!(_decoded_header_xdr.id_pool, 0);
        assert_eq!(_decoded_header_xdr.base_fee, 100);
        assert_eq!(_decoded_header_xdr.base_reserve, 100000000);
        assert_eq!(_decoded_header_xdr.max_tx_set_size, 100);
        assert_eq!(_decoded_header_xdr.ext, stellar_xdr::LedgerHeaderExt::V0);
        for decoded in _decoded_header_xdr.skip_list {
            assert_eq!(
                decoded.to_string(),
                "0000000000000000000000000000000000000000000000000000000000000000"
            );
        }
    }

    #[tokio::test]
    async fn test_get_all_claimable_balances() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let mut all_claimable_balances_request = AllClaimableBalancesRequest::new();
        all_claimable_balances_request.set_limit(2);

        let _all_claimable_balances_response = horizon_client
            .get_all_claimable_balances(&all_claimable_balances_request)
            .await;

        assert!(_all_claimable_balances_response.clone().is_ok());

        let binding = _all_claimable_balances_response.clone().unwrap();
        let predicate = binding.embedded().records()[1].claimants()[1].predicate();

        let now = Utc::now();

        let jan_first_2022 = Utc::with_ymd_and_hms(&Utc, 2022, 1, 1, 0, 0, 0).unwrap();

        assert_eq!(predicate.is_valid_claim(now), true);

        assert_eq!(predicate.is_valid_claim(jan_first_2022), false);

        assert_eq!(
            _all_claimable_balances_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .id(),
            "000000006520216af66d20d63a58534d6cbdf28ba9f2a9c1e03f8d9a756bb7d988b29bca"
        );

        assert_eq!(
            _all_claimable_balances_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .asset(),
            "native"
        );

        assert_eq!(
            _all_claimable_balances_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .amount(),
            "12.3300000"
        );

        assert_eq!(
            _all_claimable_balances_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .sponsor(),
            "GD7TMSN67PCPZ4SXQHNG4GFO4KEMGTAT6MGWQGKBPOFDY7TP2IYDYFVI"
        );

        assert_eq!(
            *_all_claimable_balances_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .last_modified_ledger(),
            1560
        );

        assert_eq!(
            _all_claimable_balances_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .last_modified_time(),
            "2023-06-14T11:38:24Z"
        );

        assert_eq!(
            *_all_claimable_balances_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .flags()
                .clawback_enabled(),
            false
        );
    }
}
