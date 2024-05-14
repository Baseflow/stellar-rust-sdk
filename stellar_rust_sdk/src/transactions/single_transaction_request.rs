use crate::models::*;

/// Represents the transaction hash.
#[derive(Default, Clone)]
pub struct TransactionHash(String);

/// Represents the absence of a transaction hash.
#[derive(Default, Clone)]
pub struct NoTransactionHash;

#[derive(Default)]
pub struct SingleTransactionRequest<T> {
    /// Transaction hash must be a hex-encoded, lowercase SHA-256, 64 char string. 
    transaction_hash: T,
}

impl SingleTransactionRequest<NoTransactionHash> {
    /// Creates a new `SingleTransactionRequest` with default parameters.
    pub fn new() -> Self {
        SingleTransactionRequest::default()
    }

    /// Sets the transaction hash for the request.
    /// 
    /// # Arguments
    /// * `transaction_hash` - A `String` specifying the operation hash.
    /// 
    pub fn set_transaction_hash(
        self,
        transaction_hash: String,
    ) -> Result<SingleTransactionRequest<TransactionHash>, String> {
        match transaction_hash.len() {
            64 => Ok(SingleTransactionRequest {transaction_hash: TransactionHash(transaction_hash)}),
            _ => Err("Transaction hash must be 64 characters long".to_string())
        }
    }
}

impl Request for SingleTransactionRequest<TransactionHash> {
    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        query.push_str(&format!("{}", self.transaction_hash.0));

        query.trim_end_matches('&').to_string()
    }

    fn build_url(&self, base_url: &str) -> String {
        // This URL is not built with query parameters, but with the transaction hash as addition to the path.
        // Therefore there is no `?` but a `/` in the formatted string.
        format!(
            "{}/{}/{}",
            base_url,
            super::TRANSACTIONS_PATH,
            self.get_query_parameters()
        )
    }
}