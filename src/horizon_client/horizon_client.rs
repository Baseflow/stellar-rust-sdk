use crate::{
    accounts::prelude::{
        AccountsRequest, AccountsResponse, SingleAccountRequest, SingleAccountsResponse,
    },
    assets::prelude::{AllAssetsRequest, AllAssetsResponse},
    claimable_balances::prelude::{
        AllClaimableBalancesRequest, AllClaimableBalancesResponse, SingleClaimableBalanceRequest,
        SingleClaimableBalanceResponse,
    },
    ledgers::{prelude::{
        LedgersRequest, LedgersResponse, SingleLedgerRequest, SingleLedgerResponse,
    }, single_ledger_request::Sequence},
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

    /// Gets an account list from the server
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

    /// Gets a single account from the server
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

    /// Gets all assets from the server
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

    /// Gets all claimable balances from the server
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

    /// Gets a single claimable balance from the server
    /// # Arguments
    /// * `self` - The Horizon client
    /// * request - The single claimable balance request
    /// # Returns
    /// The single claimable balance response
    /// # Errors
    /// Returns an error if the request fails
    /// [GET /claimable_balances/{claimable_balance_id}](https://www.stellar.org/developers/horizon/reference/endpoints/claimable_balances-single.html)
    pub async fn get_single_claimable_balance(
        &self,
        request: &SingleClaimableBalanceRequest,
    ) -> Result<SingleClaimableBalanceResponse, String> {
        self.get::<SingleClaimableBalanceResponse>(request).await
    }

    /// Gets the all ledger response from the server
    /// # Arguments
    /// * `self` - The Horizon client
    /// * request - The ledgers request
    /// # Returns
    /// The ledgers response
    /// # Errors
    /// Returns an error if the request fails
    /// [GET /ledgers](https://www.stellar.org/developers/horizon/reference/endpoints/ledgers-all.html)
    pub async fn get_all_ledgers(
        &self,
        request: &LedgersRequest,
    ) -> Result<LedgersResponse, String> {
        self.get::<LedgersResponse>(request).await
    }

    /// Gets a single ledger from the server
    /// # Arguments
    /// * `self` - The Horizon client
    /// * request - The single ledger request
    /// # Returns
    /// The single ledger response
    /// # Errors
    /// Returns an error if the request fails
    /// [GET /ledgers/{ledger_id}](https://www.stellar.org/developers/horizon/reference/endpoints/ledgers-single.html)
    pub async fn get_single_ledger(
        &self,
        request: &SingleLedgerRequest<Sequence>,
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
        let url = request.build_url(&self.base_url);
        let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
        // println!("\n\nREQWEST RESPONSE: {:?}", response);
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
    // println!("URL: {}", url);
    // check if start with http:// or https://
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(format!("URL must start with http:// or https://: {}", url));
    }
    Url::parse(url).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use base64::{Engine, engine::general_purpose};
    use chrono::{ TimeZone, Utc};

    use crate::{
        assets::prelude::AllAssetsRequest,
        claimable_balances::prelude::SingleClaimableBalanceRequest,
        ledgers::prelude::SingleLedgerRequest,
    };

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
            _accounts_response.clone().unwrap()._embedded().records()[0].account_id(),
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7"
        );

        assert_eq!(
            _accounts_response.clone().unwrap()._embedded().records()[0].id(),
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7"
        );

        assert_eq!(
            _accounts_response.clone().unwrap()._embedded().records()[0].sequence(),
            "4380492979765248"
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].subentry_count(),
            0
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].last_modified_ledger(),
            1019913
        );

        assert_eq!(
            _accounts_response.clone().unwrap()._embedded().records()[0].last_modified_time(),
            "2023-08-15T09:46:25Z"
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0]
                .thresholds()
                .low_threshold(),
            0
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0]
                .thresholds()
                .med_threshold(),
            0
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0]
                .thresholds()
                .high_threshold(),
            0
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0]
                .flags()
                .auth_required(),
            false
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0]
                .flags()
                .auth_revocable(),
            false
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0]
                .flags()
                .auth_immutable(),
            false
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0]
                .flags()
                .auth_clawback_enabled(),
            false
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].balances()[0].balance(),
            "10000.0000000".to_string()
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].balances()[0]
                .asset_type(),
            "native".to_string()
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].balances()[0]
                .buying_liabilities(),
            "0.0000000".to_string()
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].balances()[0]
                .selling_liabilities(),
            "0.0000000".to_string()
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].signers()[0].key(),
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string()
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].signers()[0].weight(),
            1
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].signers()[0]
                .signer_type(),
            "ed25519_public_key".to_string()
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].num_sponsoring(),
            0
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].num_sponsored(),
            0
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].paging_token(),
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
                .account_id()
                .to_string(),
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7"
        );

        assert_eq!(
            _single_account_response
                .clone()
                .unwrap()
                .sequence()
                .to_string(),
            "4380492979765248"
        );

        assert_eq!(
            *_single_account_response
                .clone()
                .as_ref()
                .unwrap()
                .subentry_count(),
            0
        );

        assert_eq!(
            *_single_account_response
                .as_ref()
                .unwrap()
                .last_modified_ledger(),
            1019913
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .last_modified_time(),
            "2023-08-15T09:46:25Z"
        );

        assert_eq!(
            *_single_account_response
                .as_ref()
                .unwrap()
                .thresholds()
                .low_threshold(),
            0
        );

        assert_eq!(
            *_single_account_response
                .as_ref()
                .unwrap()
                .thresholds()
                .med_threshold(),
            0
        );

        assert_eq!(
            *_single_account_response
                .as_ref()
                .unwrap()
                .thresholds()
                .high_threshold(),
            0
        );

        assert_eq!(
            *_single_account_response
                .as_ref()
                .unwrap()
                .flags()
                .auth_required(),
            false
        );

        assert_eq!(
            *_single_account_response
                .as_ref()
                .unwrap()
                .flags()
                .auth_revocable(),
            false
        );

        assert_eq!(
            *_single_account_response
                .as_ref()
                .unwrap()
                .flags()
                .auth_immutable(),
            false
        );

        assert_eq!(
            *_single_account_response
                .as_ref()
                .unwrap()
                .flags()
                .auth_clawback_enabled(),
            false
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().balances()[0].balance(),
            "10000.0000000"
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().balances()[0].asset_type(),
            "native"
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().balances()[0].buying_liabilities(),
            "0.0000000"
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().balances()[0].selling_liabilities(),
            "0.0000000"
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().signers()[0].key(),
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7"
        );

        assert_eq!(
            *_single_account_response.as_ref().unwrap().signers()[0].weight(),
            1
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().signers()[0].type_(),
            "ed25519_public_key"
        );

        assert_eq!(
            *_single_account_response.as_ref().unwrap().num_sponsoring(),
            0
        );

        assert_eq!(
            *_single_account_response.as_ref().unwrap().num_sponsored(),
            0
        );

        assert_eq!(
            _single_account_response.as_ref().unwrap().paging_token(),
            "GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7"
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
            _all_assets_response.clone().unwrap()._embedded().records()[0].asset_type(),
            "credit_alphanum4"
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0].asset_code(),
            "0"
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0].asset_issuer(),
            "GCINFW5NLMVSE7KWH5BOVL2NTRP2HN6LSXTIL76GOVIORLFM6YN5ZTRS"
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0].paging_token(),
            "0_GCINFW5NLMVSE7KWH5BOVL2NTRP2HN6LSXTIL76GOVIORLFM6YN5ZTRS_credit_alphanum4"
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0].num_accounts(),
            0
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0]
                .num_claimable_balances(),
            0
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0].num_liquidity_pools(),
            0
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0].amount(),
            "0.0000000"
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0]
                .accounts()
                .authorized(),
            0
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0]
                .accounts()
                .authorized_to_maintain_liabilities(),
            0
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0]
                .accounts()
                .unauthorized(),
            1
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0]
                .claimable_balances_amount(),
            "0.0000000"
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0].liquidity_pools_amount(),
            "0.0000000"
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0].contracts_amount(),
            "0.0000000"
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0]
                .balances()
                .authorized(),
            "0.0000000"
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0]
                .balances()
                .authorized_to_maintain_liabilities(),
            "0.0000000"
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0]
                .balances()
                .unauthorized(),
            "1.0000000"
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0]
                .flags()
                .auth_required(),
            true
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0]
                .flags()
                .auth_revocable(),
            true
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0]
                .flags()
                .auth_immutable(),
            false
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0]
                .flags()
                .auth_clawback_enabled(),
            true
        );
    }

    #[tokio::test]
    async fn test_get_all_ledgers() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let all_ledgers_request = LedgersRequest::new()
                .set_limit(2).unwrap();

        let _all_ledgers_response = horizon_client.get_all_ledgers(&all_ledgers_request).await;

        assert!(_all_ledgers_response.clone().is_ok());

        assert_eq!(
            _all_ledgers_response.clone().unwrap()._embedded().records()[0].hash(),
            "eca856e0073dc2087249dc929ed31c09c3babfef2e687b685d0513dbe6489a18"
        );

        assert_eq!(
            _all_ledgers_response.clone().unwrap()._embedded().records()[0].prev_hash(),
            "63d98f536ee68d1b27b5b89f23af5311b7569a24faf1403ad0b52b633b07be99"
        );

        assert_eq!(
            *_all_ledgers_response.clone().unwrap()._embedded().records()[0].sequence(),
            2
        );

        assert_eq!(
            *_all_ledgers_response.clone().unwrap()._embedded().records()[0]
                .successful_transaction_count(),
            0
        );

        assert_eq!(
            _all_ledgers_response.clone().unwrap()._embedded().records()[0].paging_token(),
            "8589934592"
        );
    }

    #[tokio::test]
    async fn test_get_single_ledger() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let mut single_ledger_request = SingleLedgerRequest::new()
            .set_sequence(2).unwrap();

        let _single_ledger_response = horizon_client
            .get_single_ledger(&single_ledger_request)
            .await;

        assert!(_single_ledger_response.clone().is_ok());

        assert_eq!(
            _single_ledger_response.clone().unwrap().id(),
            "eca856e0073dc2087249dc929ed31c09c3babfef2e687b685d0513dbe6489a18"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().paging_token(),
            "8589934592"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().hash(),
            "eca856e0073dc2087249dc929ed31c09c3babfef2e687b685d0513dbe6489a18"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().prev_hash(),
            "63d98f536ee68d1b27b5b89f23af5311b7569a24faf1403ad0b52b633b07be99"
        );

        assert_eq!(*_single_ledger_response.clone().unwrap().sequence(), 2);

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .successful_transaction_count(),
            0
        );

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .failed_transaction_count(),
            0
        );

        assert_eq!(
            *_single_ledger_response.clone().unwrap().operation_count(),
            0
        );

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .tx_set_operation_count(),
            0
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().closed_at(),
            "2023-06-14T09:19:48Z"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().total_coins(),
            "100000000000.0000000"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().fee_pool(),
            "0.0000000"
        );

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .base_fee_in_stroops(),
            100
        );

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .base_reserve_in_stroops(),
            100000000
        );

        assert_eq!(
            *_single_ledger_response.clone().unwrap().max_tx_set_size(),
            100
        );

        assert_eq!(
            *_single_ledger_response.clone().unwrap().protocol_version(),
            0
        );

        let decoded_xdr_header = _single_ledger_response
            .unwrap()
            .decoded_header_xdr()
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
        let tx_set_hash_bytes = hex::decode(tx_set_hash.clone()).expect("Failed to decode hex");
        let tx_set_hash_base64 = general_purpose::STANDARD.encode(tx_set_hash_bytes.clone());

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
        let single_ledger_request = SingleLedgerRequest::new()
            .set_sequence(2).unwrap();

        let _single_ledger_response = horizon_client
            .get_single_ledger(&single_ledger_request)
            .await;

        assert!(_single_ledger_response.is_ok());

        assert_eq!(
            _single_ledger_response.clone().unwrap().id(),
            "eca856e0073dc2087249dc929ed31c09c3babfef2e687b685d0513dbe6489a18"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().paging_token(),
            "8589934592"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().hash().to_string(),
            "eca856e0073dc2087249dc929ed31c09c3babfef2e687b685d0513dbe6489a18"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().prev_hash(),
            "63d98f536ee68d1b27b5b89f23af5311b7569a24faf1403ad0b52b633b07be99"
        );

        assert_eq!(*_single_ledger_response.clone().unwrap().sequence(), 2);

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .successful_transaction_count(),
            0
        );

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .failed_transaction_count(),
            0
        );

        assert_eq!(
            *_single_ledger_response.clone().unwrap().operation_count(),
            0
        );

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .tx_set_operation_count(),
            0
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().closed_at(),
            "2023-06-14T09:19:48Z"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().total_coins(),
            "100000000000.0000000"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().fee_pool(),
            "0.0000000"
        );

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .base_fee_in_stroops(),
            100
        );

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .base_reserve_in_stroops(),
            100000000
        );

        assert_eq!(
            *_single_ledger_response.clone().unwrap().max_tx_set_size(),
            100
        );

        assert_eq!(
            *_single_ledger_response.clone().unwrap().protocol_version(),
            0
        );

        let _decoded_header_xdr = _single_ledger_response
            .unwrap()
            .decoded_header_xdr()
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

    #[tokio::test]
    async fn test_get_single_claimable_balance() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let mut single_claimable_balance_request = SingleClaimableBalanceRequest::new();
        single_claimable_balance_request.set_claimable_balance_id(
            "000000006520216af66d20d63a58534d6cbdf28ba9f2a9c1e03f8d9a756bb7d988b29bca".to_string(),
        );

        let single_claimable_balance_response = horizon_client
            .get_single_claimable_balance(&single_claimable_balance_request)
            .await;

        assert!(single_claimable_balance_response.is_ok());

        let binding = single_claimable_balance_response.clone().unwrap();

        let predicate = binding.claimants()[1].predicate();

        let now = Utc::now();

        let jan_first_2022 = Utc::with_ymd_and_hms(&Utc, 2022, 1, 1, 0, 0, 0).unwrap();

        assert_eq!(predicate.is_valid_claim(now), true);

        assert_eq!(predicate.is_valid_claim(jan_first_2022), false);

        assert_eq!(
            single_claimable_balance_response
                .clone()
                .unwrap()
                .id()
                .to_string(),
            "000000006520216af66d20d63a58534d6cbdf28ba9f2a9c1e03f8d9a756bb7d988b29bca"
        );

        assert_eq!(
            single_claimable_balance_response
                .clone()
                .unwrap()
                .asset()
                .to_string(),
            "native"
        );

        assert_eq!(
            single_claimable_balance_response
                .clone()
                .unwrap()
                .amount()
                .to_string(),
            "12.3300000"
        );

        assert_eq!(
            single_claimable_balance_response
                .clone()
                .unwrap()
                .sponsor()
                .to_string(),
            "GD7TMSN67PCPZ4SXQHNG4GFO4KEMGTAT6MGWQGKBPOFDY7TP2IYDYFVI"
        );

        assert_eq!(
            *single_claimable_balance_response
                .clone()
                .unwrap()
                .last_modified_ledger(),
            1560
        );

        assert_eq!(
            single_claimable_balance_response
                .clone()
                .unwrap()
                .last_modified_time()
                .to_string(),
            "2023-06-14T11:38:24Z"
        );

        assert_eq!(
            *single_claimable_balance_response
                .clone()
                .unwrap()
                .flags()
                .clawback_enabled(),
            false
        );

        assert_eq!(
            single_claimable_balance_response
                .clone()
                .unwrap()
                .paging_token()
                .to_string(),
            "1560-000000006520216af66d20d63a58534d6cbdf28ba9f2a9c1e03f8d9a756bb7d988b29bca"
        );
    }
}
