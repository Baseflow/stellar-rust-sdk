use crate::models::*;

/// SingleClaimableBalanceRequest is the struct that implements the type for the /claimable_balances endpoint to get a single claimable balance
/// [More Details](https://laboratory.stellar.org/#explorer?resource=claimable_balances&endpoint=single&network=test "Single Claimable Balance")
#[derive(Debug)]
pub struct SingleClaimableBalanceRequest {
    /// Claimable Balance ID
    /// [Stellar Documentation](https://developers.stellar.org/api/resources/claimablebalances/single/ "Claimable Balance ID")
    claimable_balance_id: Option<String>,
}

impl Request for SingleClaimableBalanceRequest {
    /// Creates a new request object
    /// # Returns
    /// A new request object
    /// [SingleClaimableBalanceRequest](struct.SingleClaimableBalanceRequest.html)
    fn new() -> Self {
        SingleClaimableBalanceRequest {
            claimable_balance_id: None,
        }
    }

    /// Gets the relative URL for the request
    /// # Returns
    /// The relative URL for the request
    fn get_path(&self) -> &str {
        "/claimable_balances/"
    }

    /// Gets the query parameters for the request
    /// # Returns
    /// The query parameters for the request
    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        if let Some(claimable_balance_id) = &self.claimable_balance_id {
            query.push_str(&format!("{}", claimable_balance_id));
        }
        query
    }

    /// Builds the URL for the request
    /// # Arguments
    /// * `self` - The request object
    /// * `base_url` - The base URL for the Horizon server
    /// # Returns
    /// The URL for the request
    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}{}{}",
            base_url,
            self.get_path(),
            self.get_query_parameters()
        )
    }

    /// Returns the type of request
    /// # Returns
    /// The type of request
    /// [RequestType](../enum.RequestType.html)
    /// [More Details](https://laboratory.stellar.org/#explorer?resource=claimable_balances&endpoint=single&network=test "Single Claimable Balance")
    fn validate(&self) -> Result<(), String> {

        // TODO: Validate claimable_balance_id

        Ok(())
    }
}

/// Returns the claimable balance ID
/// # Arguments
/// * `self` - The request object
/// # Returns
/// The claimable balance ID
impl SingleClaimableBalanceRequest {
    pub fn set_claimable_balance_id(&mut self, claimable_balance_id: String) -> &mut Self {
        self.claimable_balance_id = Some(claimable_balance_id);
        self
    }
}