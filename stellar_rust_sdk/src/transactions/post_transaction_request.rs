use crate::models::*;

/// Represents the transaction envelope XDR.
#[derive(Default, Clone)]
pub struct TransactionEnvelope(String);

/// Represents the absence of a transaction envelope XDR.
#[derive(Default, Clone)]
pub struct NoTransactionEnvelope;

#[derive(Default)]
pub struct PostTransactionRequest<T> {
    /// A base64-encoded string containing the transaction envelope XDR.
    transaction_envelope_xdr: T,
}

impl PostTransactionRequest<NoTransactionEnvelope> {
    /// Creates a new `PostTransactionRequest` with default parameters.
    pub fn new() -> Self {
        PostTransactionRequest::default()
    }

    /// Sets the transaction envelope for the request.
    ///
    /// # Arguments
    /// * `transaction_envelope_xdr` - A `String` specifying the transaction envelope XDR.
    ///
    pub fn set_transaction_envelope_xdr(
        self,
        transaction_envelope_xdr: impl Into<String>,
    ) -> Result<PostTransactionRequest<TransactionEnvelope>, String> {
        Ok(PostTransactionRequest {
            transaction_envelope_xdr: TransactionEnvelope(transaction_envelope_xdr.into()),
        })
    }
}

impl PostRequest for PostTransactionRequest<TransactionEnvelope> {
    fn get_body(&self) -> Vec<(String, String)> {
        // Return a vector containing a tuple with a key/value pair, to be used in the request's formdata.
        // Since the request has one parameter, a vector with only 1 tuple is returned.
        vec![(
            "tx".to_string(),
            self.transaction_envelope_xdr.0.to_string(),
        )]
    }

    fn build_url(&self, base_url: &str) -> String {
        // This URL is not built with query parameters, but uses formdata, which is POSTed to the transactions API endpoint.
        format!("{}/{}", base_url, super::TRANSACTIONS_PATH)
    }
}
