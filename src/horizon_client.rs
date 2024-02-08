use crate::{
    accounts::prelude::*,
    assets::prelude::{AllAssetsRequest, AllAssetsResponse},
    claimable_balances::prelude::{
        AllClaimableBalancesRequest, AllClaimableBalancesResponse, ClaimableBalanceId,
        SingleClaimableBalanceRequest, SingleClaimableBalanceResponse,
    },
    ledgers::{
        prelude::{LedgersRequest, LedgersResponse, SingleLedgerRequest, SingleLedgerResponse},
        single_ledger_request::Sequence,
    },
    models::{Request, Response},
};
use reqwest;
use url::Url;

pub struct HorizonClient {
    /// The URL of the Horizon API server
    base_url: String,
}

impl HorizonClient {
    /// Creates a new instance of the `HorizonClient`.
    ///
    /// This constructor method initializes a new `HorizonClient` with the specified base URL
    /// for the Horizon API server. It performs validation on the provided URL to ensure it is
    /// well-formed and appropriate for establishing a connection.
    ///
    /// # Arguments
    /// * `base_url` - A `String` representing the base URL of the Horizon server.
    ///
    /// # Returns
    /// If successful, this method returns a `Result` containing the initialized `HorizonClient`
    /// instance. If the URL validation fails, it returns an error encapsulated within `Result`.
    ///
    /// # Example
    /// ```rust
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org".to_string())
    ///     .expect("Failed to create HorizonClient");
    /// ```
    pub fn new(base_url: String) -> Result<Self, String> {
        url_validate(&base_url)?;
        Ok(Self { base_url })
    }

    /// Retrieves a list of accounts filtered by specific criteria.
    ///
    /// This method retrieves a list of accounts from the Horizon server, filtering the results
    /// based on one of four categories: sponsor, signer, asset, or liquidity pool.
    ///
    /// Adheres to the <a href="https://developers.stellar.org/api/horizon/resources/list-all-accounts">List All Accounts</a>
    /// endpoint.
    ///
    /// # Arguments
    /// * `request` - A reference to an implementation of the [`ValidAccountsRequest`] trait,
    /// which specifies the filter criteria for the account list request.
    ///
    /// # Returns
    /// If successful, this method returns a `Result` containing an [`AccountsResponse`],
    /// which encapsulates the list of accounts retrieved from the server.
    /// In case of a failure in the request, it returns an error encapsulated within `Result`.
    ///
    /// # Example
    /// To use this method, create an instance of [`AccountsRequest`] and set at least
    /// one of the four filter options. For example, filtering by signer:
    ///
    /// ```rust
    /// # use stellar_rs::accounts::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org".to_string();
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = AccountsRequest::new()
    ///     .set_signer_filter("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7")
    ///     .unwrap();
    ///
    /// let response: Result<AccountsResponse, String> = horizon_client
    ///     .get_account_list(&request)
    ///     .await;
    ///
    /// // Access the account details
    /// for record in response?._embedded().records() {
    ///     println!("Account ID: {}", record.account_id());
    ///     // Further processing...
    ///  }
    ///
    /// # Ok({})
    /// # }
    pub async fn get_account_list(
        &self,
        request: &impl ValidAccountsRequest,
    ) -> Result<AccountsResponse, String> {
        self.get::<AccountsResponse>(request).await
    }

    /// Retrieves detailed information for a specific account from the Horizon server.
    ///
    /// This asynchronous method is designed to fetch information for a single account on the Horizon server.
    /// It requires a [`SingleAccountRequest`] with the account ID to be queried.
    ///
    /// Adheres to the <a href="https://developers.stellar.org/api/horizon/resources/retrieve-an-account">Retrieve an Account</a>
    /// endpoint.
    ///
    /// # Arguments
    /// * `request` - A reference to a [`SingleAccountRequest`] instance, containing the
    /// account ID for which details are to be fetched.
    ///
    /// # Returns
    ///
    /// On success, returns a `Result` wrapping a [`SingleAccountResponse`], which includes the
    /// detailed information of the requested account. If the request fails, it returns an error
    /// encapsulated within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`SingleAccountRequest`] and set the
    /// account ID of the target account.
    ///
    /// ```
    /// # use stellar_rs::accounts::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org".to_string();
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = SingleAccountRequest::new()
    ///     .set_account_id("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string())
    ///     .unwrap();
    ///
    /// let response = horizon_client.get_single_account(&request).await;
    ///
    /// // Access the account details
    /// println!("Account ID: {}", response?.account_id());
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_single_account(
        &self,
        request: &SingleAccountRequest<AccountId>,
    ) -> Result<SingleAccountResponse, String> {
        self.get::<SingleAccountResponse>(request).await
    }

    /// Retrieves a list of all assets.
    ///
    /// This asynchronous method fetches a complete list of assets.
    /// It requires a [`AllAssetsRequest`] to specify  optional query parameters
    /// such as filters by `asset_code` or `asset_issuer`.
    ///
    /// Adheres to the <a href="https://developers.stellar.org/api/horizon/resources/list-all-assets">List all Assets</a>
    /// endpoint.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`AllAssetsRequest`] instance, containing the
    /// parameters for the assets list request.
    ///
    /// # Returns
    ///
    /// On success, this method returns a `Result` wrapping an [`AllAssetsResponse`], which includes
    /// the comprehensive list of assets retrieved from the Horizon server. If the request
    /// encounters an issue, an error is returned within `Result`.
    ///
    /// # Example
    /// To use this method, create an instance of [`AllAssetsRequest`], set any desired
    /// filters or parameters and pass
    ///
    /// ```
    /// # use stellar_rs::assets::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org".to_string();
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = AllAssetsRequest::new()
    ///     .set_asset_code("USD")
    ///     .unwrap();
    ///
    /// let response = horizon_client.get_all_assets(&request).await;
    ///
    /// // Access asset details
    /// for asset in response?._embedded().records() {
    ///     println!("Asset Code: {}", asset.asset_code());
    ///     // Further processing...
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_all_assets(
        &self,
        request: &AllAssetsRequest,
    ) -> Result<AllAssetsResponse, String> {
        self.get::<AllAssetsResponse>(request).await
    }

    /// Retrieves all claimable balances.
    ///
    /// This asynchronous method queries the Horizon server for all claimable balances. It
    /// requires a [`AllClaimableBalancesRequest`] to specify the query parameters.
    ///
    /// Adheres to the <a href="https://developers.stellar.org/api/horizon/resources/list-all-claimable-balances">List All Claimable Balances</a>
    /// endpoint.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`AllClaimableBalancesRequest`] instance, which contains
    /// the parameters for the claimable balances request.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an [`AllClaimableBalancesResponse`] with the list of all
    /// claimable balances if successful. In case of a failure, it returns an error within `Result`.
    ///
    /// # Example
    /// To use this method, create an instance of [`AllClaimableBalancesRequest`], set any desired
    /// filters or parameters and pass
    ///
    /// ```
    /// # use stellar_rs::claimable_balances::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org".to_string();
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = AllClaimableBalancesRequest::new()
    ///     .set_sponsor("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string())
    ///     .unwrap();
    ///
    /// let response = horizon_client.get_all_claimable_balances(&request).await;
    ///
    /// match response {
    ///     Ok(all_balances) => {
    ///         for balance in all_balances.embedded().records() {
    ///             println!("Balance ID: {}", balance.id());
    ///             // Further processing...
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Error fetching claimable balances: {}", e),
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_all_claimable_balances(
        &self,
        request: &AllClaimableBalancesRequest,
    ) -> Result<AllClaimableBalancesResponse, String> {
        self.get::<AllClaimableBalancesResponse>(request).await
    }

    /// Retrieves detailed information about a specific claimable balance from the Horizon server.
    ///
    /// This asynchronous method is used to fetch detailed information about a single claimable
    /// balance from the Horizon server. It requires a [`SingleClaimableBalanceRequest`] that
    /// includes the unique identifier of the claimable balance to be retrieved.
    ///
    /// Adheres to the <a href="https://developers.stellar.org/api/horizon/resources/retrieve-a-claimable-balance">Retrieve a Claimable Balance</a>
    /// endpoint.
    ///
    /// # Arguments
    /// * `request` - A reference to a [`SingleClaimableBalanceRequest`] instance containing the
    /// unique ID of the claimable balance to be fetched.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`SingleClaimableBalanceResponse`],
    /// which includes detailed information about the requested claimable balance. If the request
    /// fails, it returns an error within `Result`.
    ///
    /// # Example
    /// To use this method, create an instance of [`SingleClaimableBalanceRequest`]
    /// with the specific claimable balance ID.
    ///
    /// ```
    /// # use stellar_rs::claimable_balances::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org".to_string();
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    ///  let request = SingleClaimableBalanceRequest::new()
    ///    .set_claimable_balance_id("000000006520216af66d20d63a58534d6cbdf28ba9f2a9c1e03f8d9a756bb7d988b29bca".to_string());
    ///
    /// let response = horizon_client.get_single_claimable_balance(&request).await;
    ///
    /// // Access the details of the claimable balance
    /// if let Ok(balance_response) = response {
    ///     println!("Balance Amount: {}", balance_response.amount());
    ///     // Further processing...
    /// }
    ///
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_single_claimable_balance(
        &self,
        request: &SingleClaimableBalanceRequest<ClaimableBalanceId>,
    ) -> Result<SingleClaimableBalanceResponse, String> {
        self.get::<SingleClaimableBalanceResponse>(request).await
    }

    /// Retrieves a list of all ledgers.
    ///
    /// This asynchronous method is designed to fetch list of ledgers
    /// from the Horizon server. It requires a [`LedgersRequest`] to specify the parameters
    /// for the ledger retrieval.
    ///
    /// Adheres to the <a href="https://developers.stellar.org/api/horizon/resources/list-all-ledgers">List All Ledgers</a>
    /// endpoint.
    ///
    /// # Arguments
    /// * `request` - A reference to a [`LedgersRequest`] instance, specifying the query
    /// parameters for retrieving the ledgers.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`LedgersResponse`],
    /// which includes the list of all ledgers obtained from the Horizon server. If the request
    /// fails, it returns an error within `Result`.
    ///
    /// # Example
    /// To use this method, create an instance of [`LedgersRequest`], set any
    /// desired pagination parameters.
    ///
    /// ```
    /// # use stellar_rs::ledgers::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org".to_string();
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = LedgersRequest::new()
    ///     .set_limit(2)
    ///     .unwrap();
    ///
    /// let response = horizon_client.get_all_ledgers(&request).await;
    /// // Process the response
    /// match response {
    ///     Ok(ledgers_response) => {
    ///         for ledger in ledgers_response._embedded().records() {
    ///             println!("Ledger ID: {}", ledger.id());
    ///             // Further processing...
    ///         }
    ///     }
    ///     Err(e) => println!("Error parsing response: {}", e),
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_all_ledgers(
        &self,
        request: &LedgersRequest,
    ) -> Result<LedgersResponse, String> {
        self.get::<LedgersResponse>(request).await
    }

    /// Retrieves detailed information for a specific ledger from the Horizon server.
    ///
    /// This asynchronous method fetches details of a single ledger from the Horizon server.
    /// It requires a [`SingleLedgerRequest`] parameterized with `Sequence`, which includes the sequence number
    /// of the ledger to be retrieved.
    ///
    /// Adheres to the <a href="https://developers.stellar.org/api/horizon/resources/retrieve-a-ledger">Retrieve a Ledger</a>
    /// endpoint.
    ///
    /// # Arguments
    /// * `request` - A reference to a [`SingleLedgerRequest<Sequence>`] instance, containing the
    ///   sequence number of the ledger for which details are to be fetched.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a [`SingleLedgerResponse`], which includes detailed
    /// information about the requested ledger. If the request fails, it returns an error
    /// encapsulated within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`SingleLedgerRequest`] and set the
    /// sequence number of the ledger to be queried.
    ///
    /// ```
    /// # use stellar_rs::ledgers::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org".to_string();
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = SingleLedgerRequest::new()
    ///     .set_sequence(2).unwrap();
    ///
    /// let response = horizon_client.get_single_ledger(&request).await;
    ///
    /// if let Ok(ledger) = response {
    ///     println!("Ledger ID: {}", ledger.id());
    ///     // Additional processing...
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_single_ledger(
        &self,
        request: &SingleLedgerRequest<Sequence>,
    ) -> Result<SingleLedgerResponse, String> {
        self.get::<SingleLedgerResponse>(request).await
    }

    /// Sends a GET request to the Horizon server and retrieves a specified response type.
    ///
    /// This internal asynchronous method is designed to handle various GET requests to the
    /// Horizon server. It is generic over the response type, allowing for flexibility in
    /// handling different types of responses as dictated by the caller. This method performs
    /// key tasks such as request validation, URL construction, sending the request, and
    /// processing the received response.
    ///
    /// # Type Parameters
    ///
    /// * `Response` - Defines the expected response type. This type must implement the
    /// [`Response`] trait.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to an object implementing the [`Request`] trait. It contains
    /// specific details about the GET request to be sent.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the response of type [`Response`] if the request is
    /// successful. In case of failure (e.g., network issues, server errors), it returns an
    /// error encapsulated as a `String`.
    ///
    /// # Example Usage
    ///
    /// This function is typically not called directly but through other specific methods of
    /// the `HorizonClient` that define the type of request and response.
    ///
    /// # Remarks
    ///
    /// As a core utility function within `HorizonClient`, it centralizes the logic of sending
    /// GET requests and handling responses. Modifications or enhancements to the request or
    /// response handling logic should be implemented here to maintain consistency across the
    /// client's interface.
    ///
    async fn get<R: Response>(&self, request: &impl Request) -> Result<R, String> {
        // Construct the URL with potential query parameters.
        let url = request.build_url(&self.base_url);

        // Send the request and await the response.
        let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;

        // Process the response and return the result.
        let result: R = handle_response(response).await?;
        Ok(result)
    }
}

/// Handles the response received from an HTTP request made to the Horizon server.
///
/// This asynchronous internal function processes the [`reqwest::Response`] obtained from a
/// GET request. It is generic over the type `Response` which must implement the
/// [`Response`] trait. The function primarily checks the HTTP status code of the
/// response. If the status is `OK`, it attempts to deserialize the response body into
/// the specified `Response` type. For other status codes, it treats the response as an
/// error message.
///
/// # Type Parameters
///
/// * `Response` - The type into which the response body is to be deserialized. This type
/// must implement the [`Response`] trait.
///
/// # Arguments
///
/// * `response` - The [`reqwest::Response`] object obtained from the HTTP request.
///
/// # Returns
///
/// On success (HTTP status `OK`), returns a `Result` containing the deserialized
/// `Response`. If deserialization fails, or if the HTTP status is not `OK`, it returns
/// an error encapsulated as a `String`.
///
/// # Example Usage
/// This function is not intended to be called directly. It is designed to be called
/// exclusively by the [`HorizonClient::get`](crate::horizon_client::HorizonClient::get) function.
///
/// # Errors
///
/// Errors can arise from various situations, such as:
/// - Non-`OK` HTTP status codes.
/// - Failure in reading the response body.
/// - Deserialization errors when converting the response body into the `Response` type.
///
async fn handle_response<R: Response>(response: reqwest::Response) -> Result<R, String> {
    match response.status() {
        reqwest::StatusCode::OK => {
            let _response = response.text().await.map_err(|e| e.to_string())?;
            R::from_json(_response)
        }
        _ => {
            let response = response.text().await.map_err(|e| e.to_string())?;
            Err(response)
        }
    }
}

/// Validates the format of a given URL.
///
/// This function is an internal utility for validating the format of a URL.
/// It is typically invoked by [`HorizonClient::new`](crate::horizon_client::HorizonClient::new) to ensure that the URL
/// provided for initializing the client is correctly formatted. The function checks if
/// the URL begins with "http://" or "https://", and attempts to parse it using the `Url`
/// type from the `url` crate.
///
/// # Arguments
///
/// * `url` - A string slice representing the URL to be validated.
///
/// # Returns
///
/// Returns `Ok(())` if the URL is valid, indicating that the URL has the correct format
/// and scheme. If the URL is invalid, it returns an `Err` with a message describing
/// the issue.
///
/// # Example Usage
///
/// While this function is primarily used internally by [`HorizonClient::new`](crate::horizon_client::HorizonClient::new),
/// it can also be utilized in scenarios where URL validation is necessary before further
/// processing or usage.
///
fn url_validate(url: &str) -> Result<(), String> {
    // Check if the URL starts with http:// or https://
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(format!("URL must start with http:// or https://: {}", url));
    }

    // Attempt to parse the URL to validate its format.
    Url::parse(url).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose, Engine};
    use chrono::{TimeZone, Utc};

    use crate::{
        accounts::prelude::AccountsRequest, assets::prelude::AllAssetsRequest,
        claimable_balances::prelude::SingleClaimableBalanceRequest,
        ledgers::prelude::SingleLedgerRequest,
    };

    use super::*;

    static ACCOUNT_ID: &str = "GCAHCEGRUI7FFAQE3DBQWV7ULMQHFBUIVRZC4R2VISREAY6D52Z2NODN";
    static SEQUENCE: &str = "131988639973376";
    static LAST_MODIFIED_TIME: &str = "2024-02-08T14:25:14Z";
    static LAST_MODIFIED_LEDGER: u64 = 30731;

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
        let accounts_request = AccountsRequest::new()
            .set_signer_filter(ACCOUNT_ID)
            .unwrap()
            .set_limit(10)
            .unwrap();

        // call the get_account_list method to retrieve the account list response
        let _accounts_response: Result<AccountsResponse, String> =
            horizon_client.get_account_list(&accounts_request).await;

        assert!(_accounts_response.is_ok());

        assert_eq!(
            _accounts_response.clone().unwrap()._embedded().records()[0].account_id(),
            ACCOUNT_ID
        );

        assert_eq!(
            _accounts_response.clone().unwrap()._embedded().records()[0].id(),
            ACCOUNT_ID
        );

        assert_eq!(
            _accounts_response.clone().unwrap()._embedded().records()[0].sequence(),
            SEQUENCE
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].subentry_count(),
            0
        );

        assert_eq!(
            *_accounts_response.clone().unwrap()._embedded().records()[0].last_modified_ledger(),
            LAST_MODIFIED_LEDGER
        );

        assert_eq!(
            _accounts_response.clone().unwrap()._embedded().records()[0].last_modified_time(),
            LAST_MODIFIED_TIME
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
            ACCOUNT_ID.to_string()
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
            ACCOUNT_ID.to_string()
        );
    }

    #[tokio::test]
    async fn test_get_single_account() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let single_account_request = SingleAccountRequest::new()
            .set_account_id(ACCOUNT_ID.to_string())
            .unwrap();

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
            ACCOUNT_ID
        );

        assert_eq!(
            _single_account_response
                .clone()
                .unwrap()
                .sequence()
                .to_string(),
            SEQUENCE
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
            LAST_MODIFIED_LEDGER
        );

        assert_eq!(
            _single_account_response
                .as_ref()
                .unwrap()
                .last_modified_time(),
            LAST_MODIFIED_TIME
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
            ACCOUNT_ID
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
            ACCOUNT_ID
        );
    }

    #[tokio::test]
    async fn test_get_all_assets() {
        let asset_type = "credit_alphanum4";
        let asset_code = "0";
        let asset_issuer = "GD63TVEPI5CV67GVWGPDTP3ZDNA4VH3VCH6XEPMEMRZSWYHMYNW5GKM2";
        let paging_token =
            "0_GD63TVEPI5CV67GVWGPDTP3ZDNA4VH3VCH6XEPMEMRZSWYHMYNW5GKM2_credit_alphanum4";
        let num_accounts = 0;
        let amount = "0.0000000";
        let num_authorized = 0;
        let num_unauthorized = 0;
        let balances_authorized = "0.0000000";
        let balances_unauthorized = "0.0000000";

        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let all_assets_request: AllAssetsRequest = AllAssetsRequest::new().set_limit(1).unwrap();

        let _all_assets_response = horizon_client.get_all_assets(&all_assets_request).await;

        assert!(_all_assets_response.is_ok());

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0].asset_type(),
            asset_type
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0].asset_code(),
            asset_code
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0].asset_issuer(),
            asset_issuer
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0].paging_token(),
            paging_token
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0].paging_token(),
            &format!("{}_{}_{}", asset_code, asset_issuer, asset_type)
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0].num_accounts(),
            num_accounts
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
            amount
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0]
                .accounts()
                .authorized(),
            num_authorized
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0]
                .accounts()
                .authorized_to_maintain_liabilities(),
            1
        );

        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0]
                .accounts()
                .unauthorized(),
            num_unauthorized
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
            balances_authorized
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0]
                .balances()
                .authorized_to_maintain_liabilities(),
            "1.0000000"
        );

        assert_eq!(
            _all_assets_response.clone().unwrap()._embedded().records()[0]
                .balances()
                .unauthorized(),
            balances_unauthorized
        );

        let auth_required = true;
        assert_eq!(
            *_all_assets_response.clone().unwrap()._embedded().records()[0]
                .flags()
                .auth_required(),
            auth_required
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
        let hash = "f96c4021adc1ae496c662f4f97143e499a9548f541c64bb2401a1b1701de5150";
        let prev_hash = "63d98f536ee68d1b27b5b89f23af5311b7569a24faf1403ad0b52b633b07be99";

        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let all_ledgers_request = LedgersRequest::new().set_limit(2).unwrap();

        let _all_ledgers_response = horizon_client.get_all_ledgers(&all_ledgers_request).await;

        assert!(_all_ledgers_response.clone().is_ok());

        assert_eq!(
            _all_ledgers_response.clone().unwrap()._embedded().records()[0].hash(),
            hash
        );

        assert_eq!(
            _all_ledgers_response.clone().unwrap()._embedded().records()[0].prev_hash(),
            prev_hash
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
        let id = "f96c4021adc1ae496c662f4f97143e499a9548f541c64bb2401a1b1701de5150";
        let hash = "f96c4021adc1ae496c662f4f97143e499a9548f541c64bb2401a1b1701de5150";
        let prev_hash = "63d98f536ee68d1b27b5b89f23af5311b7569a24faf1403ad0b52b633b07be99";
        let closed_at = "2024-02-06T17:32:26Z";
        let closed_at_timepoint = 1707240746;

        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let single_ledger_request = SingleLedgerRequest::new().set_sequence(2).unwrap();

        let _single_ledger_response = horizon_client
            .get_single_ledger(&single_ledger_request)
            .await;

        assert!(_single_ledger_response.clone().is_ok());

        assert_eq!(_single_ledger_response.clone().unwrap().id(), id);

        assert_eq!(
            _single_ledger_response.clone().unwrap().paging_token(),
            "8589934592"
        );

        assert_eq!(_single_ledger_response.clone().unwrap().hash(), hash);

        assert_eq!(
            _single_ledger_response.clone().unwrap().prev_hash(),
            prev_hash
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
            closed_at
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
            stellar_xdr::curr::TimePoint(closed_at_timepoint)
        );

        assert_eq!(
            decoded_xdr_header.ext,
            stellar_xdr::curr::LedgerHeaderExt::V0
        );
        for decoded in decoded_xdr_header.skip_list {
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
        let all_claimable_balances_request =
            AllClaimableBalancesRequest::new().set_limit(4).unwrap();

        let _all_claimable_balances_response = horizon_client
            .get_all_claimable_balances(&all_claimable_balances_request)
            .await;

        assert!(_all_claimable_balances_response.clone().is_ok());

        let binding = _all_claimable_balances_response.clone().unwrap();
        let predicate = binding.embedded().records()[1].claimants()[0].predicate();

        let jan_first_2024 = Utc::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap();
        let valid_date = Utc::with_ymd_and_hms(&Utc, 2024, 2, 10, 0, 0, 0).unwrap();

        assert_eq!(predicate.is_valid(jan_first_2024), true);
        assert_eq!(predicate.is_valid(valid_date), true);

        assert_eq!(
            _all_claimable_balances_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .id(),
            "000000000a12cd57c169a34e7794bdcdf2d093fab135c59ea599e2d1233d7a53f26c1464"
        );

        assert_eq!(
            _all_claimable_balances_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .asset(),
            "USDC:GAKNDFRRWA3RPWNLTI3G4EBSD3RGNZZOY5WKWYMQ6CQTG3KIEKPYWAYC"
        );

        assert_eq!(
            _all_claimable_balances_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .amount(),
            "0.0010000"
        );

        assert_eq!(
            _all_claimable_balances_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .sponsor(),
            "GCENYNAX2UCY5RFUKA7AYEXKDIFITPRAB7UYSISCHVBTIAKPU2YO57OA"
        );

        assert_eq!(
            *_all_claimable_balances_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .last_modified_ledger(),
            591
        );

        assert_eq!(
            _all_claimable_balances_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .last_modified_time(),
            "2024-02-06T18:25:07Z"
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

        let single_claimable_balance_request = SingleClaimableBalanceRequest::new()
            .set_claimable_balance_id(
                "000000000a12cd57c169a34e7794bdcdf2d093fab135c59ea599e2d1233d7a53f26c1464"
                    .to_string(),
            );

        let single_claimable_balance_response = horizon_client
            .get_single_claimable_balance(&single_claimable_balance_request)
            .await;

        assert!(single_claimable_balance_response.is_ok());

        let binding = single_claimable_balance_response.clone().unwrap();
        let predicate = binding.claimants()[0].predicate();

        let jan_first_2024 = Utc::with_ymd_and_hms(&Utc, 2024, 1, 1, 0, 0, 0).unwrap();
        let valid_date = Utc::with_ymd_and_hms(&Utc, 2024, 2, 10, 0, 0, 0).unwrap();

        assert_eq!(predicate.is_valid(jan_first_2024), true);
        assert_eq!(predicate.is_valid(valid_date), true);

        assert_eq!(
            single_claimable_balance_response
                .clone()
                .unwrap()
                .id()
                .to_string(),
            "000000000a12cd57c169a34e7794bdcdf2d093fab135c59ea599e2d1233d7a53f26c1464"
        );

        assert_eq!(
            single_claimable_balance_response
                .clone()
                .unwrap()
                .asset()
                .to_string(),
            "USDC:GAKNDFRRWA3RPWNLTI3G4EBSD3RGNZZOY5WKWYMQ6CQTG3KIEKPYWAYC"
        );

        assert_eq!(
            single_claimable_balance_response
                .clone()
                .unwrap()
                .amount()
                .to_string(),
            "0.0010000"
        );

        assert_eq!(
            single_claimable_balance_response
                .clone()
                .unwrap()
                .sponsor()
                .to_string(),
            "GCENYNAX2UCY5RFUKA7AYEXKDIFITPRAB7UYSISCHVBTIAKPU2YO57OA"
        );

        assert_eq!(
            *single_claimable_balance_response
                .clone()
                .unwrap()
                .last_modified_ledger(),
            591
        );

        assert_eq!(
            single_claimable_balance_response
                .clone()
                .unwrap()
                .last_modified_time()
                .to_string(),
            "2024-02-06T18:25:07Z"
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
            "591-000000000a12cd57c169a34e7794bdcdf2d093fab135c59ea599e2d1233d7a53f26c1464"
        );
    }
}
