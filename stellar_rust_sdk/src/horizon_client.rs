use crate::{
    accounts::prelude::*,
    assets::prelude::{AllAssetsRequest, AllAssetsResponse},
    claimable_balances::{
        all_claimable_balances_request::AllClaimableBalancesRequest,
        prelude::{AllClaimableBalancesResponse, ClaimableBalance},
        single_claimable_balance_request::{ClaimableBalanceId, SingleClaimableBalanceRequest},
    },
    effects::prelude::*,
    fee_stats::{fee_stats_request::FeeStatsRequest, response::FeeStatsResponse},
    ledgers::{
        prelude::{Ledger, LedgersRequest, LedgersResponse, SingleLedgerRequest},
        single_ledger_request::Sequence,
    },
    liquidity_pools::{
        all_liquidity_pools_request::AllLiquidityPoolsRequest,
        prelude::{
            AllLiquidityPoolsResponse, LiquidityPool, LiquidityPoolId, SingleLiquidityPoolRequest,
        },
    },
    models::{PostRequest, Request, Response},
    offers::prelude::*,
    operations::{
        operations_for_account_request::OperationsForAccountRequest,
        prelude::{
            AllOperationsRequest, OperationResponse, OperationsForLedgerRequest,
            OperationsForLiquidityPoolRequest, OperationsForTransactionRequest,
        },
        response::Operation,
        single_operation_request::{OperationId, SingleOperationRequest},
    },
    order_book::{
        details_request::{BuyingAsset, DetailsRequest, SellingAsset},
        response::DetailsResponse,
    },
    paths::prelude::*,
    payments::prelude::*,
    trade_aggregations::prelude::*,
    trades::prelude::*,
    transactions::prelude::*,
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
    /// let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org")
    ///     .expect("Failed to create HorizonClient");
    /// ```
    pub fn new(base_url: impl Into<String>) -> Result<Self, String> {
        let base_url = base_url.into();
        url_validate(&base_url)?;
        Ok(Self { base_url })
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

    /// Sends a POST request to the Horizon server and retrieves a specified response type.
    ///
    /// This internal asynchronous method is designed to handle various POST requests to the
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
    /// * `request` - A reference to an object implementing the [`PostRequest`] trait. It contains
    /// specific details about the POST request to be sent.
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
    /// POST requests and handling responses. Modifications or enhancements to the request or
    /// response handling logic should be implemented here to maintain consistency across the
    /// client's interface.
    ///
    async fn post<R: Response>(&self, request: &impl PostRequest) -> Result<R, String> {
        // Construct the URL.
        let url = request.build_url(&self.base_url);

        // Send the request and await the response.
        // The vector of tuples (containing the key/value pairs) returned by the `get_body()` method can
        // be passed directly to `reqwest`s `form()` method, which will automatically create a valid
        // formdata body for the request.
        let response = reqwest::Client::new()
            .post(&url)
            .form(&request.get_body())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        // Process the response and return the result.
        let result: R = handle_response(response).await?;

        Ok(result)
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
    /// # let base_url = "https://horizon-testnet.stellar.org";
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
    /// for record in response?.embedded().records() {
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
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = SingleAccountRequest::new()
    ///     .set_account_id("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7")
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
    ) -> Result<Account, String> {
        self.get::<Account>(request).await
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
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = AllAssetsRequest::new()
    ///     .set_asset_code("USD")
    ///     .unwrap();
    ///
    /// let response = horizon_client.get_all_assets(&request).await;
    ///
    /// // Access asset details
    /// for asset in response?.embedded().records() {
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
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = AllClaimableBalancesRequest::new()
    ///     .set_sponsor("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7")
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
    /// On successful execution, returns a `Result` containing a [`ClaimableBalance`],
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
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    ///  let request = SingleClaimableBalanceRequest::new()
    ///    .set_claimable_balance_id("000000006520216af66d20d63a58534d6cbdf28ba9f2a9c1e03f8d9a756bb7d988b29bca");
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
    ) -> Result<ClaimableBalance, String> {
        self.get::<ClaimableBalance>(request).await
    }

    /// Retrieves a list of effects for a specific account from the Horizon server.
    ///
    /// This asynchronous method fetches a list of effects for a specific account from the Horizon server.
    /// It requires an [`EffectsForAccountRequest`] to specify the account ID and optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`EffectsForAccountRequest`] instance, containing the account ID
    /// and optional query parameters for the effects request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`EffectsForAccountResponse`], which includes
    /// the list of effects obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Example
    /// To use this method, create an instance of [`EffectsForAccountRequest`] and set the account ID and any
    /// desired query parameters.
    ///
    /// ```
    /// # use stellar_rs::effects::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = EffectsForAccountRequest::new()
    ///    .set_account_id("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7");
    ///
    /// let response = horizon_client.get_effects_for_account(&request).await;
    ///
    /// // Access the effects
    /// if let Ok(effects_response) = response {
    ///    for effect in effects_response.embedded().records() {
    ///       println!("Effect ID: {}", effect.id());
    ///      // Further processing...
    ///   }
    /// }
    ///
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_effects_for_account(
        &self,
        request: &EffectsForAccountRequest,
    ) -> Result<EffectsResponse, String> {
        self.get::<EffectsResponse>(request).await
    }

    /// Retrieves a list of effects for a specific account from the Horizon server.
    ///
    /// This asynchronous method fetches a list of effects for a specific account from the Horizon server.
    /// It requires an [`EffectsForLiquidityPoolsRequest`] to specify the account ID and optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`EffectsForLiquidityPoolsRequest`] instance, containing the account ID
    /// and optional query parameters for the effects request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`EffectsForLiquidityPoolResponse`], which includes
    /// the list of effects obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Example
    /// To use this method, create an instance of [`EffectsForLiquidityPoolsRequest`] and set the account ID and any
    /// desired query parameters.
    ///
    /// ```
    /// # use stellar_rs::effects::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = EffectsForAccountRequest::new()
    ///    .set_account_id("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7");
    ///
    /// let response = horizon_client.get_effects_for_account(&request).await;
    ///
    /// // Access the effects
    /// if let Ok(effects_response) = response {
    ///    for effect in effects_response.embedded().records() {
    ///       println!("Effect ID: {}", effect.id());
    ///      // Further processing...
    ///   }
    /// }
    ///
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_effects_for_liquidity_pools(
        &self,
        request: &EffectsForLiquidityPoolRequest,
    ) -> Result<EffectsResponse, String> {
        self.get::<EffectsResponse>(request).await
    }

    /// Retrieves a list of effects for a specific operation from the Horizon server.
    ///
    /// This asynchronous method fetches a list of effects for a specific operation from the Horizon server.
    /// It requires an [`EffectsForOperationRequest`] to specify the operation ID and optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`EffectsForOperationRequest`] instance, containing the operation ID
    /// and optional query parameters for the effects request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`EffectsResponse`], which includes
    /// the list of effects obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Example
    /// To use this method, create an instance of [`EffectsForOperationRequest`] and set the operation ID and any
    /// desired query parameters.
    ///
    /// ```
    /// # use stellar_rs::effects::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = EffectsForOperationRequest::new()
    ///   .set_operation_id("123");
    ///     
    /// let response = horizon_client.get_effects_for_operation(&request).await;
    ///
    /// // Access the effects
    /// if let Ok(effects_response) = response {
    ///     for effect in effects_response.embedded().records() {
    ///         println!("Effect ID: {}", effect.id());
    ///     // Further processing...
    ///     }
    /// }
    ///
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_effects_for_operation(
        &self,
        request: &EffectsForOperationRequest,
    ) -> Result<EffectsResponse, String> {
        self.get::<EffectsResponse>(request).await
    }

    /// Retrieves a list of effects for a specific transaction from the Horizon server.
    ///
    /// This asynchronous method fetches a list of effects for a specific transaction from the Horizon server.
    /// It requires an [`EffectForTransactionRequest`] to specify the transaction hash and optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`EffectForTransactionRequest`] instance, containing the transaction hash
    /// and optional query parameters for the effects request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`EffectsResponse`], which includes
    /// the list of effects obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Example
    /// To use this method, create an instance of [`EffectForTransactionRequest`] and set the transaction hash and any
    /// desired query parameters.
    ///
    /// ```
    /// # use stellar_rs::effects::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = EffectForTransactionRequest::new()
    ///  .set_transaction_hash("transaction_hash");
    ///
    /// let response = horizon_client.get_effects_for_transaction(&request).await;
    ///
    /// // Access the effects
    /// if let Ok(effects_response) = response {
    ///     for effect in effects_response.embedded().records() {
    ///         println!("Effect ID: {}", effect.id());
    ///     // Further processing...
    ///     }
    /// }
    ///
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_effects_for_transaction(
        &self,
        request: &EffectForTransactionRequest,
    ) -> Result<EffectsResponse, String> {
        self.get::<EffectsResponse>(request).await
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
    /// # let base_url = "https://horizon-testnet.stellar.org";
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
    ///         for ledger in ledgers_response.embedded().records() {
    ///             println!("Ledger ID: {}", ledger.id());
    ///             // Further processing...
    ///         }
    ///     }
    ///     Err(e) => println!("Error parsing response: {}", e),
    /// }
    /// # Ok({})
    /// # }
    /// ```
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
    /// Returns a `Result` containing a [`Ledger`], which includes detailed
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
    /// # let base_url = "https://horizon-testnet.stellar.org";
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
    ) -> Result<Ledger, String> {
        self.get::<Ledger>(request).await
    }

    /// Retrieves a list of all effects from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all effects from the Horizon server.
    /// It requires an [`AllEffectsRequest`] to specify the optional query parameters.
    ///
    /// Adheres to the <a href="https://developers.stellar.org/api/horizon/resources/list-all-effects">Retrieve a Ledger</a>
    /// endpoint.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to an [`AllEffectsRequest`] instance, containing the
    /// parameters for the effects request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`AllEffectsResponse`], which includes
    /// the list of all effects obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`AllEffectsRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::effects::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = AllEffectsRequest::new()
    ///    .set_limit(2).unwrap();
    ///
    /// let response = horizon_client.get_all_effects(&request).await;
    ///
    /// // Access the effects
    /// if let Ok(effects_response) = response {
    ///   for effect in effects_response.embedded().records() {
    ///     println!("Effect ID: {}", effect.id());
    ///    // Further processing...
    ///  }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_all_effects(
        &self,
        request: &AllEffectsRequest,
    ) -> Result<EffectsResponse, String> {
        self.get::<EffectsResponse>(request).await
    }

    /// Fetches effects associated with a specific ledger from the Stellar Horizon API.
    ///
    /// This asynchronous method retrieves effects for a given ledger, facilitating detailed analysis
    /// and insight into the various operations and changes that occurred within that ledger. It requires
    /// a [`EffectsForLedgerRequest`], which includes options  for pagination, record limits,
    /// and sorting order, among others.
    ///
    /// Adheres to <a href="https://developers.stellar.org/api/horizon/resources/retrieve-a-ledgers-effects">Retrieve a Ledgers's Effects</a> endpoint.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`EffectsForLedgerRequest`] instance, specifying the ledger sequence
    ///   and optional parameters such as cursor, limit, and order for the query.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an [`AllEffectsResponse`], which encompasses a collection of effects
    /// related to the requested ledger. If the operation fails, it returns an error message encapsulated
    /// within `Result`.
    ///
    /// # Usage
    /// To utilize this method, instantiate an `EffectsForLedgerRequest` with the desired parameters.
    ///
    /// ```
    /// # use stellar_rs::effects::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)?;
    /// let mut request = EffectsForLedgerRequest::new()
    ///     .set_sequence(&125)
    ///     .set_limit(2).unwrap();
    ///
    /// let response = horizon_client.get_effects_for_ledger(&request).await;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub async fn get_effects_for_ledger(
        &self,
        request: &EffectsForLedgerRequest,
    ) -> Result<EffectsResponse, String> {
        self.get::<EffectsResponse>(request).await
    }

    /// Retrieves a list of all fee stats from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all fee stats from the Horizon server.
    /// It requires a [`FeeStatsRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to a [`FeeStatsRequest`] instance, containing the
    /// parameters for the fee stats request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`FeeStatsResponse`], which includes
    /// the list of all fee stats obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`FeeStatsRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::fee_stats::fee_stats_request::FeeStatsRequest;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = FeeStatsRequest::new();
    ///
    /// let response = horizon_client.get_fee_stats(&request).await;
    ///
    /// // Access the fee stats
    /// if let Ok(fee_stats_response) = response {
    ///  println!("Max Fee: {:?}", fee_stats_response.max_fee());
    /// // Further processing...
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_fee_stats(
        &self,
        request: &FeeStatsRequest,
    ) -> Result<FeeStatsResponse, String> {
        self.get::<FeeStatsResponse>(request).await
    }

    /// Retrieves detailed information for a specific offer from the Horizon server.
    ///
    /// This asynchronous method fetches details of a single offer from the Horizon server.
    /// It requires a [`SingleOfferRequest`] parameterized with `OfferId`, which includes the ID
    /// of the offer to be retrieved.
    ///
    /// Adheres to the <a href="https://developers.stellar.org/network/horizon/resources/get-offer-by-offer-id">Retrieve An Offer endpoint</a>
    /// endpoint.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to a [`SingleOfferRequest<OfferId>`] instance, containing the
    ///   id of the offer for which details are to be fetched.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an [`Offer`], which includes detailed
    /// information about the requested offer. If the request fails, it returns an error
    /// encapsulated within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`SingleOfferRequest`] and set the
    /// id of the offer to be queried.
    ///
    /// ```
    /// # use stellar_rs::offers::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = SingleOfferRequest::new()
    ///     .set_offer_id("1") // example offer ID
    ///     .unwrap();
    ///
    /// let response = horizon_client.get_single_offer(&request).await;
    ///
    /// if let Ok(offer) = response {
    ///     println!("Offer ID: {}", offer.id());
    ///     // Additional processing...
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_single_offer(
        &self,
        request: &SingleOfferRequest<OfferId>,
    ) -> Result<OfferResponse, String> {
        self.get::<OfferResponse>(request).await
    }

    /// Retrieves a list of all offers from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all offers from the Horizon server.
    /// It requires an [`AllOffersRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`AllOffersRequest`] instance, containing the
    /// parameters for the offers request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`OfferResponse`], which includes
    /// the list of all offers obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`AllOffersRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::offers::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = AllOffersRequest::new()
    ///   .set_limit(2).unwrap();
    ///
    /// let response = horizon_client.get_all_offers(&request).await;
    ///
    /// // Access the offers
    /// if let Ok(offers_response) = response {
    ///     for offer in offers_response.embedded().records() {
    ///         println!("Offer ID: {}", offer.id());
    ///         // Further processing...
    ///     }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_all_offers(
        &self,
        request: &AllOffersRequest,
    ) -> Result<AllOffersResponse, String> {
        self.get::<AllOffersResponse>(request).await
    }

    /// Retrieves a list of all offers from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all offers an account currently has open from the Horizon server.
    /// It requires an [`OffersForAccountRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`OffersForAccountRequest`] instance, containing the
    /// parameters for the request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`AllOffersResponse`], which includes
    /// the list of all offers currently open for an account, obtained from the Horizon server.
    /// If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`OffersForAccountRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    pub async fn get_offers_for_account(
        &self,
        request: &OffersForAccountRequest<OfferAccountId>,
    ) -> Result<AllOffersResponse, String> {
        self.get::<AllOffersResponse>(request).await
    }

    /// Retrieves a list of all operations from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all operations from the Horizon server.
    /// It requires an [`AllOperationsRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`AllOperationsRequest`] instance, containing the
    /// parameters for the operations request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`OperationResponse`], which includes
    /// the list of all operations obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`AllOperationsRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::operations::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = AllOperationsRequest::new()
    ///   .set_limit(2).unwrap();
    ///
    /// let response = horizon_client.get_all_operations(&request).await;
    ///
    /// // Access the operations
    /// if let Ok(operations_response) = response {
    ///     for operation in operations_response.embedded().records() {
    ///         println!("Operation ID: {}", operation.id());
    ///         // Further processing...
    ///     }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_all_operations(
        &self,
        request: &AllOperationsRequest,
    ) -> Result<OperationResponse, String> {
        self.get::<OperationResponse>(request).await
    }

    /// Retrieves detailed information for a specific operation from the Horizon server.
    ///
    /// This asynchronous method fetches details of a single operation from the Horizon server.
    /// It requires a [`SingleOperationRequest`] parameterized with `OperationId`, which includes the unique identifier
    /// of the operation to be retrieved.
    ///
    /// # Arguments
    /// * `request` - A reference to a [`SingleOperationRequest`] instance containing the unique ID of the operation to be fetched.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an [`Operation`], which includes detailed information about the requested operation.
    /// If the request fails, it returns an error encapsulated within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`SingleOperationRequest`] and set the unique ID of the operation to be queried.
    ///
    /// ```
    /// # use stellar_rs::operations::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = SingleOperationRequest::new()
    ///    .set_operation_id("459561504769");
    ///
    /// let response = horizon_client.get_single_operation(&request).await;
    ///
    /// if let Ok(operation) = response {
    ///    println!("Operation ID: {}", operation.id());
    ///   // Additional processing...
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_single_operation(
        &self,
        request: &SingleOperationRequest<OperationId>,
    ) -> Result<Operation, String> {
        self.get::<Operation>(request).await
    }

    /// Retrieves a list of all operations for an account from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all operations for an account from the Horizon server.
    /// It requires an [`OperationsForAccountRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`OperationsForAccountRequest`] instance, containing the
    /// parameters for the operations for account request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`OperationResponse`], which includes
    /// the list of all operations obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`OperationsForAccountRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::operations::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = OperationsForAccountRequest::new()
    ///   .set_limit(2).unwrap();
    ///
    /// let response = horizon_client.get_operations_for_account(&request).await;
    ///
    /// // Access the payments
    /// if let Ok(operations_for_account_response) = response {
    ///   for operation in operations_for_account_response.embedded().records() {
    ///    println!("operation ID: {}", operation.id());
    ///  // Further processing...
    /// }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_operations_for_account(
        &self,
        request: &OperationsForAccountRequest,
    ) -> Result<OperationResponse, String> {
        self.get::<OperationResponse>(request).await
    }

    /// Retrieves a list of all operations for a ledger from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all operations for a ledger from the Horizon server.
    /// It requires an [`OperationsForLedgerRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`OperationsForLedgerRequest`] instance, containing the
    /// parameters for the operations for ledger request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`OperationResponse`], which includes
    /// the list of all operations obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`OperationsForLedgerRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::operations::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = OperationsForLedgerRequest::new()
    ///   .set_limit(2).unwrap();
    ///
    /// let response = horizon_client.get_operations_for_ledger(&request).await;
    ///
    /// // Access the payments
    /// if let Ok(operations_for_ledger_response) = response {
    ///   for operation in operations_for_ledger_response.embedded().records() {
    ///    println!("operation ID: {}", operation.id());
    ///  // Further processing...
    /// }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_operations_for_ledger(
        &self,
        request: &OperationsForLedgerRequest,
    ) -> Result<OperationResponse, String> {
        self.get::<OperationResponse>(request).await
    }

    /// Retrieves a list of all operations for a specific liquidity pool from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all operations for a specific liquidity pool from the Horizon server.
    /// It requires an [`OperationsForLiquidityPoolRequest`] to specify the liquidity pool ID and optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`OperationsForLiquidityPoolRequest`] instance, containing the liquidity pool ID
    /// and optional query parameters for the operations for liquidity pool request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`OperationResponse`], which includes
    /// the list of all operations obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`OperationsForLiquidityPoolRequest`] and set the liquidity pool ID and any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::operations::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = OperationsForLiquidityPoolRequest::new()
    ///  .set_liquidity_pool_id("000000006520216af66d20d63a58534d6cbdf28ba9f2a9c1e03f8d9a756bb7d988b29bca");
    ///
    /// let response = horizon_client.get_operations_for_liquidity_pool(&request).await;
    ///
    /// // Access the operations
    /// if let Ok(operations_for_liquidity_pool_response) = response {
    ///  for operation in operations_for_liquidity_pool_response.embedded().records() {
    ///
    ///   println!("Operation ID: {}", operation.id());
    /// // Further processing...
    /// }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_operations_for_liquidity_pool(
        &self,
        request: &OperationsForLiquidityPoolRequest,
    ) -> Result<OperationResponse, String> {
        self.get::<OperationResponse>(request).await
    }

    /// Retrieves a list of all operations for a specific transaction from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all operations for a specific transaction from the Horizon server.
    /// It requires an [`OperationsForTransactionRequest`] to specify the transaction hash and optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`OperationsForTransactionRequest`] instance, containing the transaction hash
    /// and optional query parameters for the operations for transaction request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`OperationResponse`], which includes
    /// the list of all operations obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`OperationsForTransactionRequest`] and set the transaction hash and any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::operations::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = OperationsForTransactionRequest::new()
    ///  .set_transaction_hash("b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020");
    ///
    /// let response = horizon_client.get_operations_for_transaction(&request).await;
    ///
    /// // Access the operations
    /// if let Ok(operations_for_transaction_response) = response {
    ///     for operation in operations_for_transaction_response.embedded().records() {
    ///         println!("Operation ID: {}", operation.id());
    ///         // Further processing...
    ///     }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_operations_for_transaction(
        &self,
        request: &OperationsForTransactionRequest,
    ) -> Result<OperationResponse, String> {
        self.get::<OperationResponse>(request).await
    }

    /// Retrieves a list of order book details from the Horizon server.
    ///
    /// This asynchronous method fetches a list of order book details from the Horizon server.
    /// It requires a [`DetailsRequest`] to specify the parameters for the order book details request.
    ///
    /// # Arguments
    /// * `request` - A reference to a [`DetailsRequest`] instance, containing the parameters for the order book details request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`DetailsResponse`], which includes the list of order book details obtained from the Horizon server.
    /// If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`DetailsRequest`] and set any desired filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::order_book::prelude::*;
    /// # use stellar_rs::models::prelude::*;
    /// # use stellar_rs::horizon_client::HorizonClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// # let details_request = DetailsRequest::new()
    /// # .set_buying_asset(AssetType::Native)
    /// # .unwrap()
    /// # .set_selling_asset(AssetType::Alphanumeric4(AssetData {
    /// #     asset_code: "USDC".to_string(),
    /// #     asset_issuer: "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5"
    /// #         .to_string(),
    /// # }))
    /// # .unwrap();
    ///
    /// let response = horizon_client.get_order_book_details(&details_request).await;
    ///
    /// assert!(response.is_ok());
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_order_book_details(
        &self,
        request: &DetailsRequest<SellingAsset, BuyingAsset>,
    ) -> Result<DetailsResponse, String> {
        self.get::<DetailsResponse>(request).await
    }

    /// Retrieves a list of trade aggregations from the Horizon server.
    ///
    /// This asynchronous method fetches a list of trade aggregations from the Horizon server.
    /// It requires a [`TradeAggregationsRequest`] to specify the parameters for the trade aggregations request.
    ///
    /// # Arguments
    /// * `request` - A reference to a [`TradeAggregationsRequest`] instance, containing the parameters for the order book details request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`AllTradeAggregationsResponse`], which includes the list of order book details obtained from the Horizon server.
    /// If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`TradeAggregationsRequest`] and set any desired filters or parameters.
    ///
    /// ```rust
    /// use stellar_rs::horizon_client::HorizonClient;
    /// use stellar_rs::trade_aggregations::prelude::*;
    /// use stellar_rs::models::prelude::*;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org")?;
    ///
    /// // Example: Fetching trade aggregations
    /// let request = TradeAggregationsRequest::new()
    ///     .set_base_asset(AssetType::Native).unwrap()
    ///     .set_counter_asset(AssetType::Alphanumeric4(AssetData {
    ///         asset_code: "USDC".to_string(),
    ///         asset_issuer: "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5".to_string(),
    ///     })).unwrap()
    ///     .set_resolution(Resolution(ResolutionData::Duration604800000)).unwrap();
    /// let response = horizon_client.get_trade_aggregations(&request).await?;
    ///
    /// // Process the response...
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub async fn get_trade_aggregations(
        &self,
        request: &TradeAggregationsRequest<BaseAsset, CounterAsset, Resolution>,
    ) -> Result<AllTradeAggregationsResponse, String> {
        self.get::<AllTradeAggregationsResponse>(request).await
    }

    /// Retrieves a list of all trades from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all trades from the Horizon server.
    /// It requires an [`AllTradesRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`AllTradesRequest`] instance, containing the
    ///     parameters for the trades request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`AllTradesResponse`], which includes
    /// the list of all trades obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`AllTradesRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::trades::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = AllTradesRequest::new();
    ///
    /// let response = horizon_client.get_all_trades(&request).await;
    ///
    /// // Access the trades
    /// if let Ok(trades_response) = response {
    ///     for trade in trades_response.embedded().records() {
    ///         println!("Trade ID: {}", trade.id());
    ///         // Further processing...
    ///     }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_all_trades(
        &self,
        request: &AllTradesRequest,
    ) -> Result<AllTradesResponse, String> {
        self.get::<AllTradesResponse>(request).await
    }

    /// Retrieves a list of all trades for a given account from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all trades for a given account from the Horizon server.
    /// It requires an [`TradesForAccountRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`TradesForAccountRequest`] instance, containing the
    ///     parameters for the trades request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`AllTradesResponse`], which includes
    /// the list of all trades obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`TradesForAccountRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::trades::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = TradesForAccountRequest::new()
    ///    .set_account_id("GCUOMNFW7YG55YHY5S5W7FE247PWODUDUZ4SOVZFEON47KZ7AXFG6D6A")
    ///    .unwrap();
    ///
    /// let response = horizon_client.get_trades_for_account(&request).await;
    ///
    /// // Access the trades
    /// if let Ok(trades_response) = response {
    ///     for trade in trades_response.embedded().records() {
    ///         println!("Trade ID: {}", trade.id());
    ///         // Further processing...
    ///     }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_trades_for_account(
        &self,
        request: &TradesForAccountRequest<TradeAccountId>,
    ) -> Result<AllTradesResponse, String> {
        self.get::<AllTradesResponse>(request).await
    }

    /// Retrieves a list of all successful trades fulfilled by the given liquidity pool from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all successful trades fulfilled by the given liquidity pool
    /// from the Horizon server. It requires an [`TradesForLiquidityPoolRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`TradesForLiquidityPoolRequest`] instance, containing the
    ///     parameters for the trades request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`AllTradesResponse`], which includes
    /// the list of all trades obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`TradesForLiquidityPoolRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::trades::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = TradesForLiquidityPoolRequest::new()
    ///    .set_liquidity_pool_id("0b3c88caa5aeada296646c1810893e3b04cba0426cff8ff6a63cf6f35cc7f5b3")
    ///    .unwrap();
    ///
    /// let response = horizon_client.get_trades_for_liquidity_pool(&request).await;
    ///
    /// // Access the trades
    /// if let Ok(trades_response) = response {
    ///     for trade in trades_response.embedded().records() {
    ///         println!("Trade ID: {}", trade.id());
    ///         // Further processing...
    ///     }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_trades_for_liquidity_pool(
        &self,
        request: &TradesForLiquidityPoolRequest<TradeLiquidityPoolId>,
    ) -> Result<AllTradesResponse, String> {
        self.get::<AllTradesResponse>(request).await
    }

    /// Retrieves a list of all trades for a given offer from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all trades for a given offer from the Horizon server.
    /// It requires an [`TradesForOfferRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`TradesForOfferRequest`] instance, containing the
    ///     parameters for the trades request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`AllTradesResponse`], which includes
    /// the list of all trades obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`TradesForOfferRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::trades::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = TradesForOfferRequest::new()
    ///    .set_offer_id("42")
    ///    .unwrap();
    ///
    /// let response = horizon_client.get_trades_for_offer(&request).await;
    ///
    /// // Access the trades
    /// if let Ok(trades_response) = response {
    ///     for trade in trades_response.embedded().records() {
    ///         println!("Trade ID: {}", trade.id());
    ///         // Further processing...
    ///     }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_trades_for_offer(
        &self,
        request: &TradesForOfferRequest<TradeOfferId>,
    ) -> Result<AllTradesResponse, String> {
        self.get::<AllTradesResponse>(request).await
    }

    /// Fetches all liquidity pools from the Stellar Horizon API.
    ///
    /// This asynchronous method retrieves a list of all liquidity pools from the Horizon server.
    /// It requires an [`AllLiquidityPoolsRequest`] to specify optional query parameters such as
    /// filters by `asset_code` or `asset_issuer`.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`AllLiquidityPoolsRequest`] instance, containing the
    /// parameters for the liquidity pools request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`AllLiquidityPoolsResponse`], which includes
    /// the comprehensive list of liquidity pools retrieved from the Horizon server. If the request
    /// encounters an issue, an error is returned within `Result`.
    ///
    /// # Example
    /// To use this method, create an instance of [`AllLiquidityPoolsRequest`], set any desired
    /// filters or parameters and pass
    ///
    /// ```
    /// # use stellar_rs::liquidity_pools::all_liquidity_pools_request::AllLiquidityPoolsRequest;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)?;
    /// let request = AllLiquidityPoolsRequest::new()
    ///     .add_alphanumeric4_reserve("USDC", "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5");
    ///
    /// let response = horizon_client.get_all_liquidity_pools(&request).await;
    ///
    /// // Access liquidity pool details
    /// for pool in response?.embedded().records() {
    ///     println!("Pool ID: {}", pool.id());
    ///     // Further processing...
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub async fn get_all_liquidity_pools(
        &self,
        request: &AllLiquidityPoolsRequest,
    ) -> Result<AllLiquidityPoolsResponse, String> {
        self.get::<AllLiquidityPoolsResponse>(request).await
    }

    /// Retrieves detailed information for a specific liquidity pool from the Horizon server.
    ///
    /// This asynchronous method is designed to fetch detailed information about a single liquidity
    /// pool from the Horizon server. It requires a [`SingleLiquidityPoolRequest`] that includes the
    /// unique identifier of the liquidity pool to be retrieved.
    ///
    /// # Arguments
    /// * `request` - A reference to a [`SingleLiquidityPoolRequest`] instance containing the
    /// unique ID of the liquidity pool to be fetched.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`SingleLiquidityPoolResponse`],
    /// which includes detailed information about the requested liquidity pool. If the request fails,
    /// it returns an error encapsulated within `Result`.
    ///
    /// # Example
    /// To use this method, create an instance of [`SingleLiquidityPoolRequest`]
    /// with the specific liquidity pool ID.
    ///
    /// ```
    /// # use stellar_rs::liquidity_pools::single_liquidity_pool_request::SingleLiquidityPoolRequest;
    /// # use stellar_rs::horizon_client::HorizonClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)?;
    /// let request = SingleLiquidityPoolRequest::new()
    ///     .set_liquidity_pool_id("000000006520216af66d20d63a58534d6cbdf28ba9f2a9c1e03f8d9a756bb7d988b29bca").unwrap();
    ///
    /// let response = horizon_client.get_single_liquidity_pool(&request).await;
    ///
    /// // Access the details of the liquidity pool
    /// if let Ok(pool_response) = response {
    ///     println!("Pool ID: {}", pool_response.id());
    ///     // Further processing...
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub async fn get_single_liquidity_pool(
        &self,
        request: &SingleLiquidityPoolRequest<LiquidityPoolId>,
    ) -> Result<LiquidityPool, String> {
        self.get::<LiquidityPool>(request).await
    }

    /// Retrieves detailed information for a specific transaction from the Horizon server.
    ///
    /// This asynchronous method fetches details of a single transaction from the Horizon server.
    /// It requires a [`SingleTransactionRequest`] parameterized with `TransactionHash`, which includes the hash
    /// of the transaction to be retrieved.
    ///
    /// Adheres to the <a href="https://developers.stellar.org/network/horizon/api-reference/resources/retrieve-a-transaction">Retrieve a Transaction endpoint</a>
    /// endpoint.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to a [`SingleTransactionRequest<TransactionHash>`] instance, containing the
    ///   hash of the transaction for which details are to be fetched.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an [`TransactionResponse`], which includes detailed
    /// information about the requested transaction. If the request fails, it returns an error
    /// encapsulated within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`SingleTransactionRequest`] and set the
    /// hash of the transaction to be queried.
    ///
    /// ```
    /// # use stellar_rs::transactions::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = SingleTransactionRequest::new()
    ///     .set_transaction_hash("be0d59c8706e8fd525d2ab10910a55ec57323663858c65b330a3f93afb13ab0f") // example transaction hash
    ///     .unwrap();
    ///
    /// let response = horizon_client.get_single_transaction(&request).await;
    ///
    /// // Access the details of the claimable balance
    /// if let Ok(transaction_response) = response {
    ///     println!("Created at: {}", transaction_response.created_at());
    ///     // Further processing...
    /// }
    ///
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_single_transaction(
        &self,
        request: &SingleTransactionRequest<TransactionHash>,
    ) -> Result<TransactionResponse, String> {
        self.get::<TransactionResponse>(request).await
    }

    /// Retrieves a list of all transactions from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all transactions from the Horizon server.
    /// It requires an [`AllTransactionsRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`AllTransactionsRequest`] instance, containing the
    /// parameters for the transactions request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`AllTransactionsResponse`], which includes
    /// the list of all transactions obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`AllTransactionsRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::transactions::prelude::*;
    /// # use stellar_rs::models::{Request, IncludeFailed};
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = AllTransactionsRequest::new()
    ///   .set_include_failed(IncludeFailed::True).unwrap();
    ///
    /// let response = horizon_client.get_all_transactions(&request).await;
    ///
    /// // Access the transactions
    /// if let Ok(transactions_response) = response {
    ///     for transaction in transactions_response.embedded().records() {
    ///         println!("Transaction ID: {}", transaction.id());
    ///         // Further processing...
    ///     }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_all_transactions(
        &self,
        request: &AllTransactionsRequest,
    ) -> Result<AllTransactionsResponse, String> {
        self.get::<AllTransactionsResponse>(request).await
    }

    /// Retrieves a list of all transactions for a given account from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all transactions for a given account from
    /// the Horizon server. It requires an [`TransactionsForAccountRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`TransactionsForAccountRequest`] instance, containing the
    /// parameters for the transactions request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`AllTransactionsResponse`], which includes
    /// the list of all transactions obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`TransactionsForAccountRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::transactions::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = TransactionsForAccountRequest::new()
    ///     .set_account_id("GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H").unwrap()
    ///     .set_include_failed(true).unwrap();
    ///
    /// let response = horizon_client.get_transactions_for_account(&request).await;
    ///
    /// // Access the transactions
    /// if let Ok(transactions_response) = response {
    ///     for transaction in transactions_response.embedded().records() {
    ///         println!("Transaction ID: {}", transaction.id());
    ///         // Further processing...
    ///     }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_transactions_for_account(
        &self,
        request: &TransactionsForAccountRequest<TransactionsAccountId>,
    ) -> Result<AllTransactionsResponse, String> {
        self.get::<AllTransactionsResponse>(request).await
    }

    /// Retrieves a list of all transactions in a given ledger from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all transactions in a given ledger from
    /// the Horizon server. It requires an [`TransactionsForLedgerRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`TransactionsForLedgerRequest`] instance, containing the
    /// parameters for the transactions request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`AllTransactionsResponse`], which includes
    /// the list of all transactions obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`TransactionsForLedgerRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::transactions::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = TransactionsForLedgerRequest::new()
    ///     .set_ledger_sequence("539").unwrap()
    ///     .set_include_failed(true).unwrap();
    ///
    /// let response = horizon_client.get_transactions_for_ledger(&request).await;
    ///
    /// // Access the transactions
    /// if let Ok(transactions_response) = response {
    ///     for transaction in transactions_response.embedded().records() {
    ///         println!("Transaction ID: {}", transaction.id());
    ///         // Further processing...
    ///     }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_transactions_for_ledger(
        &self,
        request: &TransactionsForLedgerRequest<TransactionsLedgerId>,
    ) -> Result<AllTransactionsResponse, String> {
        self.get::<AllTransactionsResponse>(request).await
    }

    /// Retrieves a list of all transactions referencing a given liquidity pool from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all transactions referencing a given liquidity pool from
    /// the Horizon server. It requires an [`TransactionsForLiquidityPoolRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`TransactionsForLiquidityPoolRequest`] instance, containing the
    /// parameters for the transactions request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`AllTransactionsResponse`], which includes
    /// the list of all transactions obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`TransactionsForLiquidityPoolRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::transactions::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = TransactionsForLiquidityPoolRequest::new()
    ///     .set_liquidity_pool_id("0066b15f5d0dc0be771209c33f3e4126383e58183a598eae8b3813024c6a6d10").unwrap()
    ///     .set_include_failed(true).unwrap();
    ///
    /// let response = horizon_client.get_transactions_for_liquidity_pool(&request).await;
    ///
    /// // Access the transactions
    /// if let Ok(transactions_response) = response {
    ///     for transaction in transactions_response.embedded().records() {
    ///         println!("Transaction ID: {}", transaction.id());
    ///         // Further processing...
    ///     }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_transactions_for_liquidity_pool(
        &self,
        request: &TransactionsForLiquidityPoolRequest<TransactionsLiquidityPoolId>,
    ) -> Result<AllTransactionsResponse, String> {
        self.get::<AllTransactionsResponse>(request).await
    }

    /// Retrieves payment paths from the Horizon server.
    ///
    /// This asynchronous method fetches a list of payment paths from
    /// the Horizon server. It requires an [`FindPaymentsPathRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`FindPaymentsPathRequest`] instance, containing the
    /// parameters for the paths request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`PathsResponse`], which includes
    /// the list of the payment paths obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`FindPaymentsPathRequest`] and set any desired
    /// filters or parameters.
    pub async fn get_find_payment_paths(
        &self,
        request: &FindPaymentsPathRequest<DestinationAsset, DestinationAmount, SourceAccount>,
    ) -> Result<PathsResponse, String> {
        self.get::<PathsResponse>(request).await
    }

    /// Retrieves a list of strict receive payment paths from the Horizon server.
    ///
    /// This asynchronous method fetches a list of strict receive payment paths from
    /// the Horizon server. It requires an [`ListStrictReceivePaymentPathsRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`ListStrictReceivePaymentPathsRequest`] instance, containing the
    /// parameters for the paths request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`PathsResponse`], which includes
    /// the list of the strict receive payment paths obtained from the Horizon server.
    /// If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`ListStrictReceivePaymentPathsRequest`] and set any desired
    /// filters or parameters.
    pub async fn get_list_strict_receive_payment_paths(
        &self,
        request: &ListStrictReceivePaymentPathsRequest<DestinationAsset, DestinationAmount, Source>,
    ) -> Result<PathsResponse, String> {
        self.get::<PathsResponse>(request).await
    }

    /// Retrieves a list of strict send payment paths from the Horizon server.
    ///
    /// This asynchronous method fetches a list of strict send payment paths from
    /// the Horizon server. It requires an [`ListStrictSendPaymentPathsRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`ListStrictSendPaymentPathsRequest`] instance, containing the
    /// parameters for the paths request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing an [`PathsResponse`], which includes
    /// the list of the strict send payment paths obtained from the Horizon server.
    /// If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`ListStrictSendPaymentPathsRequest`] and set any desired
    /// filters or parameters.
    pub async fn get_list_strict_send_payment_paths(
        &self,
        request: &ListStrictSendPaymentPathsRequest<SourceAsset, SourceAmount, Destination>,
    ) -> Result<PathsResponse, String> {
        self.get::<PathsResponse>(request).await
    }

    /// Retrieves a list of all payments from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all payments from the Horizon server.
    /// It requires an [`AllPaymentsRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`AllPaymentsRequest`] instance, containing the
    /// parameters for the payments request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`PaymentsResponse`], which includes
    /// the list of all payments obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`AllPaymentsRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::payments::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = AllPaymentsRequest::new()
    ///  .set_limit(2).unwrap();
    ///
    /// let response = horizon_client.get_all_payments(&request).await;
    ///
    /// // Access the payments
    /// if let Ok(payments_response) = response {
    ///    for payment in payments_response.embedded().records() {
    ///       println!("Payment ID: {}", payment.id());
    ///      // Further processing...
    /// }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_all_payments(
        &self,
        request: &AllPaymentsRequest,
    ) -> Result<PaymentsResponse, String> {
        self.get::<PaymentsResponse>(request).await
    }

    /// Retrieves a list of all payments for an account from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all payments for an account from the Horizon server.
    /// It requires an [`PaymentsForAccountRequest`] to specify the optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`PaymentsForAccountRequest`] instance, containing the
    /// parameters for the payments for account request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`PaymentsResponse`], which includes
    /// the list of all payments obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`PaymentsForAccountRequest`] and set any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::payments::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = PaymentsForAccountRequest::new()
    ///  .set_limit(2).unwrap();
    ///
    /// let response = horizon_client.get_payments_for_account(&request).await;
    ///
    /// // Access the payments
    /// if let Ok(payments_response) = response {
    ///   for payment in payments_response.embedded().records() {
    ///   println!("Payment ID: {}", payment.id());
    /// // Further processing...
    /// }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_payments_for_account(
        &self,
        request: &PaymentsForAccountRequest,
    ) -> Result<PaymentsResponse, String> {
        self.get::<PaymentsResponse>(request).await
    }

    /// Retrieves a list of all payments for a specific ledger from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all payments for a specific ledger from the Horizon server.
    /// It requires an [`PaymentsForLedgerRequest`] to specify the ledger sequence number and optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`PaymentsForLedgerRequest`] instance, containing the
    /// parameters for the payments for ledger request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`PaymentsResponse`], which includes
    /// the list of all payments obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`PaymentsForLedgerRequest`] and set the ledger sequence number and any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::payments::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = PaymentsForLedgerRequest::new()
    /// .set_ledger_sequence("48483");
    ///
    /// let response = horizon_client.get_payments_for_ledger(&request).await;
    ///
    /// // Access the payments
    /// if let Ok(payments_response) = response {
    /// for payment in payments_response.embedded().records() {
    /// println!("Payment ID: {}", payment.id());
    ///
    /// // Further processing...
    /// }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_payments_for_ledger(
        &self,
        request: &PaymentsForLedgerRequest,
    ) -> Result<PaymentsResponse, String> {
        self.get::<PaymentsResponse>(request).await
    }

    /// Retrieves a list of all payments for a specific transaction from the Horizon server.
    ///
    /// This asynchronous method fetches a list of all payments for a specific transaction from the Horizon server.
    /// It requires an [`PaymentsForTransactionRequest`] to specify the transaction hash and optional query parameters.
    ///
    /// # Arguments
    /// * `request` - A reference to an [`PaymentsForTransactionRequest`] instance, containing the
    /// parameters for the payments for transaction request.
    ///
    /// # Returns
    ///
    /// On successful execution, returns a `Result` containing a [`PaymentsResponse`], which includes
    /// the list of all payments obtained from the Horizon server. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`PaymentsForTransactionRequest`] and set the transaction hash and any desired
    /// filters or parameters.
    ///
    /// ```
    /// # use stellar_rs::payments::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let request = PaymentsForTransactionRequest::new()
    /// .set_transaction_hash("be0d59c8706e8fd525d2ab10910a55ec57323663858c65b330a3f93afb13ab0f");
    ///
    /// let response = horizon_client.get_payments_for_transaction(&request).await;
    ///
    /// // Access the payments
    /// if let Ok(payments_response) = response {
    /// for payment in payments_response.embedded().records() {
    /// println!("Payment ID: {}", payment.id());
    ///
    /// // Further processing...
    /// }
    /// }
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn get_payments_for_transaction(
        &self,
        request: &PaymentsForTransactionRequest,
    ) -> Result<PaymentsResponse, String> {
        self.get::<PaymentsResponse>(request).await
    }

    /// Submits a transaction to the Horizon server.
    ///
    /// This asynchronous method submits a transaction to the Stellar network. It only takes a
    /// single, required parameter: the signed transaction. Refer to the Transactions page for
    /// details on how to craft a proper one. If you submit a transaction that has already been
    /// included in a ledger, this endpoint will return the same response as would have been
    /// returned for the original transaction submission. This allows for safe resubmission of
    /// transactions in error scenarios, as highlighted in the error handling guide.
    ///
    /// # Arguments
    /// * `request` - A reference to a [`PostTransactionRequest<TransactionEnvelope>`] instance, containing the
    /// signed transaction to be submitted.
    ///
    /// # Returns
    /// On successful execution, returns a `Result` containing a [`TransactionResponse`], which includes
    /// the details of the submitted transaction. If the request fails, it returns an error within `Result`.
    ///
    /// # Usage
    /// To use this method, create an instance of [`PostTransactionRequest`] and set the signed
    /// transaction to be submitted.
    /// ```
    /// # use stellar_rs::transactions::prelude::*;
    /// # use stellar_rs::models::Request;
    /// # use stellar_rs::horizon_client::HorizonClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let base_url = "https://horizon-testnet.stellar.org";
    /// # let horizon_client = HorizonClient::new(base_url)
    /// #    .expect("Failed to create Horizon Client");
    /// let signed_transaction_xdr = "AAAAAgAAAABi/B0L0JGythwN1lY0aypo19NHxvLCyO5tBEcCVvwF9wAABEwAAAAAAAAAAQAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAsAAAAAAAAAAAAAAAAQfdFrLDgzSIIugR73qs8U0ZiKbwBUclTTPh5thlbgnAFjRXhdigAAAAAAAAAAAAAAAAAA3b5KF6uk1w1fSKYLrzR8gF2lB+AHAi6oU6CaWhunAskAAAAXSHboAAAAAAAAAAAAAAAAAHfmNeMLin2aTUfxa530ZRn4zwRu7ROAQfUJeJco8HSCAAHGv1JjQAAAAAAAAAAAAAAAAAAAlRt2go9sp7E1a5ZWvr7vin4UPrFQThpQax1lOFm33AAAABdIdugAAAAAAAAAAAAAAAAAmv+knlR6JR2VqWeU0k/4FgvZ/tSV5DEY4gu0iOTKgpUAAAAXSHboAAAAAAAAAAAAAAAAANpaWLojuOtfC0cmMh+DvQTfPDrkfXhblQTdFXrGYc0bAAAAF0h26AAAAAABAAAAAACVG3aCj2ynsTVrlla+vu+KfhQ+sVBOGlBrHWU4WbfcAAAABgAAAAFURVNUAAAAANpaWLojuOtfC0cmMh+DvQTfPDrkfXhblQTdFXrGYc0bf/////////8AAAABAAAAAJr/pJ5UeiUdlalnlNJP+BYL2f7UleQxGOILtIjkyoKVAAAABgAAAAFURVNUAAAAANpaWLojuOtfC0cmMh+DvQTfPDrkfXhblQTdFXrGYc0bf/////////8AAAABAAAAANpaWLojuOtfC0cmMh+DvQTfPDrkfXhblQTdFXrGYc0bAAAAAQAAAAAAlRt2go9sp7E1a5ZWvr7vin4UPrFQThpQax1lOFm33AAAAAFURVNUAAAAANpaWLojuOtfC0cmMh+DvQTfPDrkfXhblQTdFXrGYc0bAAAJGE5yoAAAAAABAAAAANpaWLojuOtfC0cmMh+DvQTfPDrkfXhblQTdFXrGYc0bAAAAAQAAAACa/6SeVHolHZWpZ5TST/gWC9n+1JXkMRjiC7SI5MqClQAAAAFURVNUAAAAANpaWLojuOtfC0cmMh+DvQTfPDrkfXhblQTdFXrGYc0bAAAJGE5yoAAAAAAAAAAAAAAAAABKBB+2UBMP/abwcm/M1TXO+/JQWhPwkalgqizKmXyRIQx7qh6aAFYAAAAAAAAAAARW/AX3AAAAQDVB8fT2ZXF0PZqtZX9brK0kz+P4G8VKs1DkDklP6ULsvXRexXFBdH4xG8xRAsR1HJeEBH278hiBNNvUwNw6zgzGYc0bAAAAQLgZUU/oYGL7frWDQhJHhCQu9JmfqN03PrJq4/cJrN1OSUWXnmLc94sv8m2L+cxl2p0skr2Jxy+vt1Lcxkv7wAI4WbfcAAAAQHvZEVqlygIProf3jVTZohDWm2WUNrFAFXf1LctTqDCQBHph14Eo+APwrTURLLYTIvNoXeGzBKbL03SsOARWcQLkyoKVAAAAQHAvKv2/Ro4+cNh6bKQO/G9NNiUozYysGwG1GvJQkFjwy/OTsL6WBfuI0Oye84lVBVrQVk2EY1ERFhgdMpuFSg4=";
    /// let request = PostTransactionRequest::new()
    ///    .set_transaction_envelope_xdr(signed_transaction_xdr).unwrap();
    /// let response = horizon_client.post_transaction(&request).await;
    /// # Ok({})
    /// # }
    /// ```
    ///
    pub async fn post_transaction(
        &self,
        request: &PostTransactionRequest<TransactionEnvelope>,
    ) -> Result<TransactionResponse, String> {
        self.post::<TransactionResponse>(request).await
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
pub mod tests {
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
}
