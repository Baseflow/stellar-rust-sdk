// TODO: Documentation
pub mod single_transaction_request;

// TODO: Documentation
pub mod response;

// TODO: Documentation
static TRANSACTIONS_PATH: &str = "transactions";

// TODO: Documentation
pub mod prelude {
    pub use super::single_transaction_request::*;
    pub use super::response::*;
}

#[cfg(test)]
pub mod test {
    use super::prelude::*;
    use crate::{horizon_client::HorizonClient};

    // TODO: Tests
    #[tokio::test]
    async fn test() {
        let horizon_client =
            HorizonClient::new("https://horizon-testnet.stellar.org"
            .to_string())
            .unwrap();

        let request =
            SingleTransactionRequest::new()
            .set_transaction_hash("be0d59c8706e8fd525d2ab10910a55ec57323663858c65b330a3f93afb13ab0f".to_string())
            .unwrap();

        let response = horizon_client
            .get_single_transaction(&request)
            .await;

        println!("{:?}", response);
    }
}