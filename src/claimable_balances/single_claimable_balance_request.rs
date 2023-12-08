use crate::models::*;

#[derive(Default, Clone)]
pub struct ClaimableBalanceId(String);
#[derive(Default, Clone)]
pub struct NoClaimableBalanceId;

/// SingleClaimableBalanceRequest is the struct that implements the type for the /claimable_balances endpoint to get a single claimable balance
/// [More Details](https://developers.stellar.org/api/horizon/resources/retrieve-a-claimable-balance) "Single Claimable Balance")
#[derive(Default)]
pub struct SingleClaimableBalanceRequest<I> {
    claimable_balance_id: I,
}

impl SingleClaimableBalanceRequest<NoClaimableBalanceId> {
    pub fn new() -> Self {
        SingleClaimableBalanceRequest::default()
    }

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
