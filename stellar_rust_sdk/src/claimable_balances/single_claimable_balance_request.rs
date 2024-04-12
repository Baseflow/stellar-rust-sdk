use crate::models::*;

/// Represents the ID of a claimable balance.
#[derive(Default, Clone)]
pub struct ClaimableBalanceId(String);

/// Represents the absence of a claimable balance ID.
#[derive(Default, Clone)]
pub struct NoClaimableBalanceId;

/// Represents a request to retrieve information about a single claimable balance from the Stellar Horizon API.
///
/// The `SingleClaimableBalanceRequest` struct is designed for querying detailed data for a specific
/// claimable balance, identified by its unique ID. It adheres to the structure and parameters required
/// by the Horizon API for retrieving a
/// <a href="https://developers.stellar.org/api/horizon/resources/retrieve-a-claimable-balance">single claimable balance</a>.
///
/// The struct is used in conjunction with the [`HorizonClient`](crate::horizon_client::HorizonClient) to
/// make API calls to the Horizon server and fetch the desired claimable balance information.
///
/// # Usage
///
/// To create a request, instantiate a `SingleClaimableBalanceRequest` and set the claimable balance ID
/// using `set_claimable_balance_id`. Then, pass the request object to the [`HorizonClient::get_single_claimable_balance`](crate::horizon_client::HorizonClient::get_single_claimable_balance)
/// method to execute the query. The method returns a `SingleClaimableBalanceResponse` containing the details of the claimable balance.
///
/// # Example
/// ```
/// # use stellar_rs::claimable_balances::single_claimable_balance_request::SingleClaimableBalanceRequest;
/// # use stellar_rs::horizon_client::HorizonClient;
/// # use stellar_rs::models::Request;
/// #
/// # async fn fetch_single_claimable_balance() -> Result<(), Box<dyn std::error::Error>> {
/// #     let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org".to_string())?;
/// let request = SingleClaimableBalanceRequest::new()
///     .set_claimable_balance_id("00000000".to_string());  // Example claimable balance ID
///
/// let response = horizon_client.get_single_claimable_balance(&request).await?;
/// // Process the response
/// #     Ok(())
/// # }
/// ```
///
#[derive(Default)]
pub struct SingleClaimableBalanceRequest<I> {
    claimable_balance_id: I,
}

impl SingleClaimableBalanceRequest<NoClaimableBalanceId> {
    /// Creates a new `SingleClaimableBalanceRequest` with default parameters.
    pub fn new() -> Self {
        SingleClaimableBalanceRequest::default()
    }

    /// Sets the claimable balance ID for the request.
    ///
    /// # Arguments
    /// * `claimable_balance_id` - A `String` representing the claimable balance ID.
    ///
    pub fn set_claimable_balance_id(
        self,
        claimable_balance_id: String,
    ) -> SingleClaimableBalanceRequest<ClaimableBalanceId> {
        SingleClaimableBalanceRequest {
            claimable_balance_id: ClaimableBalanceId(claimable_balance_id),
        }
    }
}

impl Request for SingleClaimableBalanceRequest<ClaimableBalanceId> {
    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        query.push_str(&format!("{}", self.claimable_balance_id.0));
        query
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}/{}",
            base_url,
            super::CLAIMABLE_BALANCES_PATH,
            self.get_query_parameters()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_claimable_balance_request() {
        let request =
            SingleClaimableBalanceRequest::new().set_claimable_balance_id("00000000".to_string());

        assert_eq!(request.get_query_parameters(), "00000000".to_string());

        assert_eq!(
            request.build_url("https://horizon-testnet.stellar.org"),
            "https://horizon-testnet.stellar.org/claimable_balances/00000000".to_string()
        );
    }
}
