use crate::models::*;


#[derive(Default, Clone)]
pub struct Id(String);
#[derive(Default, Clone)]
pub struct NoId;

#[derive(Default)]
pub struct SingleClaimableBalanceRequest<I> {
    claimable_balance_id: I,
}

impl SingleClaimableBalanceRequest<NoId> {
    pub fn new() -> Self {
        SingleClaimableBalanceRequest::default()
    }
}

impl<I> SingleClaimableBalanceRequest<I> {
    pub fn set_claimable_balance_id(
        self,
        claimable_balance_id: String,
    ) -> SingleClaimableBalanceRequest<Id> {
        SingleClaimableBalanceRequest { claimable_balance_id: Id(claimable_balance_id) }
    }
}

impl Request for SingleClaimableBalanceRequest<Id> {
    fn get_path(&self) -> &str {
        "/claimable_balances/"
    }

    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        if let Some(claimable_balance_id) = &self.claimable_balance_id {
            query.push_str(&format!("{}", claimable_balance_id));
        }
        format!("/{}", query)
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::CLAIMABLE_BALANCES_PATH,            
            self.get_query_parameters()
        )
    }
}

