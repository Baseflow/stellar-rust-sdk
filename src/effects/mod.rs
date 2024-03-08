pub mod all_effects_request;
pub mod effects_for_account_request;
pub mod effects_for_ledger_request;
pub mod effects_for_liquidity_pools_request;
pub mod response;

static EFFECTS_PATH: &str = "effects";

pub mod prelude {
    pub use super::all_effects_request::*;
    pub use super::effects_for_account_request::*;
    pub use super::effects_for_ledger_request::*;
    pub use super::effects_for_liquidity_pools_request::*;
    pub use super::response::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use crate::horizon_client::HorizonClient;

    #[tokio::test]
    async fn test_get_all_effects() {
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let num_records_to_fetch = 2;

        let all_effects_request = AllEffectsRequest::new()
            .set_limit(num_records_to_fetch)
            .unwrap();
        let _all_effects_response = horizon_client.get_all_effects(&all_effects_request).await;

        assert!(_all_effects_response.clone().is_ok());

        // make sure there are actually 2 records
        assert_eq!(
            _all_effects_response
                .clone()
                .unwrap()
                .embedded()
                .records()
                .len() as u8,
            num_records_to_fetch
        );

        // test first record retrieved
        assert_eq!(
            _all_effects_response.clone().unwrap().embedded().records()[0].type_i,
            0
        );

        // test second record retrieved
        assert_eq!(
            _all_effects_response.clone().unwrap().embedded().records()[1].type_i,
            3
        );
    }

    #[tokio::test]
    async fn test_get_effects_for_account() {
        const ID: &str = "0000000459561504769-0000000001";
        const PAGING_TOKEN: &str = "459561504769-1";
        const ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";
        const RECORD_TYPE: &str = "account_created";
        const TYPE_I: u32 = 0;
        const CREATED_AT: &str = "2024-02-06T17:42:48Z";
        const STARTING_BALANCE: &str = "10000000000.0000000";
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let effects_for_account_request = EffectsForAccountRequest::new().set_limit(2).unwrap();

        let effects_for_account_response = horizon_client
            .get_effects_for_account(&effects_for_account_request)
            .await;

        assert!(&effects_for_account_response.is_ok());
        let binding = effects_for_account_response.clone().unwrap();
        let record = &binding.embedded().records()[0];

        assert_eq!(record.id(), ID);
        assert_eq!(record.paging_token(), PAGING_TOKEN);
        assert_eq!(record.account(), ACCOUNT);
        assert_eq!(record.effect_type(), RECORD_TYPE);
        assert_eq!(record.type_i(), &TYPE_I);
        assert_eq!(record.created_at(), CREATED_AT);
        assert_eq!(
            record.starting_balance().as_ref().unwrap(),
            &STARTING_BALANCE
        );
    }

    #[tokio::test]
    async fn get_effects_for_liquidity_pools() {
        const ID: &str = "0000000459561504769-0000000001";
        const PAGING_TOKEN: &str = "459561504769-1";
        const ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";
        const RECORD_TYPE: &str = "account_created";
        const TYPE_I: u32 = 0;
        const CREATED_AT: &str = "2024-02-06T17:42:48Z";
        const STARTING_BALANCE: &str = "10000000000.0000000";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let effects_for_liquidity_pools_request =
            EffectsForLiquidityPoolRequest::new().set_limit(2).unwrap();
        let effects_for_liquidity_pools_response = horizon_client
            .get_effects_for_liquidity_pools(&effects_for_liquidity_pools_request)
            .await;

        assert!(effects_for_liquidity_pools_response.is_ok());
        let binding = effects_for_liquidity_pools_response.clone().unwrap();
        let record = &binding.embedded().records()[0];

        assert_eq!(record.id(), ID);
        assert_eq!(record.paging_token(), PAGING_TOKEN);
        assert_eq!(record.account(), ACCOUNT);
        assert_eq!(record.effect_type(), RECORD_TYPE);
        assert_eq!(record.type_i(), &TYPE_I);
        assert_eq!(record.created_at(), CREATED_AT);
        assert_eq!(
            record.starting_balance().as_ref().unwrap(),
            &STARTING_BALANCE
        );

        // TODO: LEONARD FIX
        let _effects_for_liquidity_pools_request_with_id = EffectsForLiquidityPoolRequest::new()
            .set_limit(2)
            .expect("REASON")
            .set_liquidity_pool_id("0000000459561504769-0000000001".to_string());
        let effects_for_liquidity_pools_response = horizon_client
            .get_effects_for_liquidity_pools(&effects_for_liquidity_pools_request)
            .await;

        assert!(effects_for_liquidity_pools_response.clone().is_ok());
        assert_eq!(record.id(), ID);
    }

    #[tokio::test]
    async fn test_get_effects_for_ledger() {
        // found by trial and error in the Stellar laboratory
        let ledger_sequence = 125;

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let effects_for_ledger_request =
            EffectsForLedgerRequest::new().set_sequence(ledger_sequence);
        let effects_for_ledger_response = horizon_client
            .get_effects_for_ledger(&effects_for_ledger_request)
            .await;

        assert!(effects_for_ledger_response.is_ok());

        assert_eq!(
            effects_for_ledger_response
                .clone()
                .unwrap()
                .embedded()
                .records()[0]
                .id,
            "0000000536870916097-0000000001"
        );

        assert_eq!(
            effects_for_ledger_response
                .clone()
                .unwrap()
                .embedded()
                .records()[1]
                .effect_type,
            "account_debited"
        );
    }
}
