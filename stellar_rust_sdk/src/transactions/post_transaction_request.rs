use crate::models::*;

/// Represents the transaction evenlope XDR.
#[derive(Default, Clone)]
pub struct TransactionEnvelope(String);

/// Represents the absence of a transaction evenlope XDR.
#[derive(Default, Clone)]
pub struct NoTransactionEnvelope;

#[derive(Default)]
pub struct PostTransactionRequest<T> {
    /// todo serde rename -> tx
    transaction_envelope_xdr: T,
}

impl PostTransactionRequest<NoTransactionEnvelope> {
    /// Creates a new `SingleTransactionRequest` with default parameters.
    pub fn new() -> Self {
        PostTransactionRequest::default()
    }

    /// Sets the transaction hash for the request.
    /// 
    /// # Arguments
    /// * `transaction_hash` - A `String` specifying the operation hash.
    /// 
    pub fn set_transaction_envelope_xdr(
        self,
        transaction_envelope_xdr: String,
    ) -> Result<PostTransactionRequest<TransactionEnvelope>, String> {
        // // TODO: does a XDR have a fixed length?
        Ok(PostTransactionRequest {transaction_envelope_xdr: TransactionEnvelope(transaction_envelope_xdr)})
        // match transaction_envelope_xdr.len() {
        //     64 => Ok(PostTransactionRequest {transaction_envelope_xdr: TransactionEnvelope(transaction_envelope_xdr)}),
        //     _ => Err("Transaction hash must be 64 characters long".to_string())
        // }
    }
}

impl PostRequest for PostTransactionRequest<TransactionEnvelope> {
    fn get_body(&self) -> Vec<(String, String)> {
        vec![("tx".to_string(), self.transaction_envelope_xdr.0.to_string())]
    }

    fn build_url(&self, base_url: &str) -> String {
        // This URL is not built with query parameters, but with the transaction hash as addition to the path.
        // Therefore there is no `?` but a `/` in the formatted string.
        format!(
            "{}/{}",
            base_url,
            super::TRANSACTIONS_PATH
        )
    }
}