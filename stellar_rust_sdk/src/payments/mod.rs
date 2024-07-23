mod all_payments_request;
mod payments_for_account_request;
mod payments_for_ledger_request;
mod payments_for_transaction_request;
mod response;

pub mod prelude {
    pub use super::all_payments_request::*;
    pub use super::payments_for_account_request::*;
    pub use super::payments_for_ledger_request::*;
    pub use super::payments_for_transaction_request::*;
    pub use super::response::*;
}

#[cfg(test)]
pub mod test {
    use super::prelude::*;
    use crate::{horizon_client::HorizonClient, models::IncludeFailed, Paginatable};

    static ID: &str = "2314987376641";
    static PAGING_TOKEN: &str = "2314987376641";
    static TRANSACTION_SUCCESSFUL: &bool = &true;
    static SOURCE_ACCOUNT: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
    static TYPE: &str = "create_account";
    static TYPE_I: &i64 = &0;
    static CREATED_AT: &str = "2024-06-11T21:36:12Z";
    static TRANSACTION_HASH: &str =
        "b9d0b2292c4e09e8eb22d036171491e87b8d2086bf8b265874c8d182cb9c9020";
    static STARTING_BALANCE: &str = "10000000000.0000000";
    static FUNDER: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";
    static ACCOUNT: &str = "GAIH3ULLFQ4DGSECF2AR555KZ4KNDGEKN4AFI4SU2M7B43MGK3QJZNSR";
    static LEDGER_SEQUENCE: &str = "48483";

    #[tokio::test]
    async fn test_get_all_payments() {
        // Initialize horizon client
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        // construct request
        let all_payments_request: AllPaymentsRequest =
            AllPaymentsRequest::new().set_limit(1).unwrap();

        let response: Result<PaymentsResponse, String> =
            horizon_client.get_all_payments(&all_payments_request).await;

        assert!(response.is_ok());
        let binding = response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(response.id(), ID);
        assert_eq!(response.paging_token(), PAGING_TOKEN);
        assert_eq!(response.transaction_successful(), TRANSACTION_SUCCESSFUL);
        assert_eq!(response.source_account(), SOURCE_ACCOUNT);
        assert_eq!(response.type_field(), TYPE);
        assert_eq!(response.type_i(), TYPE_I);
        assert_eq!(response.created_at(), CREATED_AT);
        assert_eq!(response.transaction_hash(), TRANSACTION_HASH);
        assert_eq!(
            response.starting_balance().as_deref(),
            Some(STARTING_BALANCE)
        );
        assert_eq!(response.funder().as_deref(), Some(FUNDER));
        assert_eq!(response.account().as_deref(), Some(ACCOUNT));
    }

    #[tokio::test]
    async fn test_get_payments_for_account() {
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let payments_for_account_request: PaymentsForAccountRequest =
            PaymentsForAccountRequest::new().set_account_id(
                "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H".to_string(),
            );

        let response: Result<PaymentsResponse, String> = horizon_client
            .get_payments_for_account(&payments_for_account_request)
            .await;

        println!("{:?}", response);

        assert!(response.is_ok());
        let binding = response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(response.id(), ID);
        assert_eq!(response.paging_token(), PAGING_TOKEN);
        assert_eq!(response.transaction_successful(), TRANSACTION_SUCCESSFUL);
        assert_eq!(response.source_account(), SOURCE_ACCOUNT);
        assert_eq!(response.type_field(), TYPE);
        assert_eq!(response.type_i(), TYPE_I);
        assert_eq!(response.created_at(), CREATED_AT);
        assert_eq!(response.transaction_hash(), TRANSACTION_HASH);
        assert_eq!(
            response.starting_balance().as_deref(),
            Some(STARTING_BALANCE)
        );
        assert_eq!(response.funder().as_deref(), Some(FUNDER));
        assert_eq!(response.account().as_deref(), Some(ACCOUNT));
    }

    #[tokio::test]
    async fn test_get_payments_for_ledger() {
        static ID: &str = "208232899428355";
        static PAGING_TOKEN: &str = "208232899428355";
        static TRANSACTION_SUCCESSFUL: &bool = &true;
        static SOURCE_ACCOUNT: &str = "GAZPDM46VZKEYJR75AQ6SBASSFVBZXXOLLWBAI7FFTKJW3227LZGVRSY";
        static TYPE: &str = "payment";
        static TYPE_I: &i64 = &1;
        static CREATED_AT: &str = "2024-06-14T19:37:16Z";
        static TRANSACTION_HASH: &str =
            "14e250278575ca3979f26c048543d278192b1958a3777c59a5d87045a22e1db8";
        static ASSET_TYPE: &str = "credit_alphanum4";
        static ASSET_CODE: &str = "AUSD";
        static ASSET_ISSUER: &str = "GDIOZ6QL5L5SV5VQVEK2NYPTS7REOMWTG2FF62OQUBHNJD2FIPXJ234G";
        static FROM: &str = "GAZPDM46VZKEYJR75AQ6SBASSFVBZXXOLLWBAI7FFTKJW3227LZGVRSY";
        static TO: &str = "GACX6RVTLABDFH7JGQT2DQD5G54MA422UYLQZWYVIW6DSLRL2RJ2GTBJ";
        static AMOUNT: &str = "1.0000000";

        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org".to_string()).unwrap();

        let payments_for_ledger_request: PaymentsForLedgerRequest = PaymentsForLedgerRequest::new()
            .set_ledger_sequence(LEDGER_SEQUENCE.to_string())
            .set_include_failed(IncludeFailed::False)
            .set_limit(1)
            .unwrap();

        let response: Result<PaymentsResponse, String> = horizon_client
            .get_payments_for_ledger(&payments_for_ledger_request)
            .await;

        println!("{:?}", response);

        assert!(response.is_ok());
        let binding = response.unwrap();
        let response = &binding.embedded().records()[0];
        assert_eq!(response.id(), ID);
        assert_eq!(response.paging_token(), PAGING_TOKEN);
        assert_eq!(response.transaction_successful(), TRANSACTION_SUCCESSFUL);
        assert_eq!(response.source_account(), SOURCE_ACCOUNT);
        assert_eq!(response.type_field(), TYPE);
        assert_eq!(response.type_i(), TYPE_I);
        assert_eq!(response.created_at(), CREATED_AT);
        assert_eq!(response.transaction_hash(), TRANSACTION_HASH);
        assert_eq!(response.asset_type().as_deref(), Some(ASSET_TYPE));
        assert_eq!(response.asset_code().as_deref(), Some(ASSET_CODE));
        assert_eq!(response.asset_issuer().as_deref(), Some(ASSET_ISSUER));
        assert_eq!(response.from().as_deref(), Some(FROM));
        assert_eq!(response.to().as_deref(), Some(TO));
        assert_eq!(response.amount().as_deref(), Some(AMOUNT));
    }
}
