use crate::{models::{Order, Request}, BuildQueryParametersExt, Paginatable};
use stellar_rust_sdk_derive::Pagination;

/// Represents a request to fetch effect data from the Stellar Horizon API.
///
/// `EffectForTransactionRequest` is a struct used to construct queries for retrieving information about effects
/// from the Horizon server. It includes parameters that allow for pagination control and sorting
/// of the effect records.
///
/// # Usage
/// Instances of `EffectForTransactionRequest` are created and optionally configured using the builder pattern.
/// Once the desired parameters are set, the request can be passed to the Horizon client to fetch
/// effect data.
///
/// # Example
/// ```rust
/// # use stellar_rs::effects::effects_for_transaction_request::EffectForTransactionRequest;
/// # use stellar_rs::models::*;
/// # use stellar_rust_sdk_derive::Pagination;
/// # use stellar_rs::Paginatable;
///
/// let request = EffectForTransactionRequest::new()
///     .set_transaction_hash("transaction_hash".to_string())
///     .set_cursor(1234).unwrap()
///     .set_limit(20).unwrap()
///     .set_order(Order::Desc);
///
/// // The request can now be used with a Horizon client to fetch effects.
/// ```
#[derive(Default, Pagination)]
pub struct EffectForTransactionRequest {
    /// The transaction hash of the transaction of the effect
    transaction_hash: Option<String>,
    /// A pointer to a specific location in a collection of responses, derived from the
    ///   `paging_token` value of a record. Used for pagination control in the API response.
    cursor: Option<u32>,
    /// Specifies the maximum number of records to be returned in a single response.
    ///   The range for this parameter is from 1 to 200. The default value is set to 10.
    limit: Option<u8>,
    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    ///   and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    order: Option<Order>,
}

impl EffectForTransactionRequest {
    /// Creates a new `EffectForTransactionRequest` with default parameters.
    pub fn new() -> Self {
        EffectForTransactionRequest::default()
    }

    /// Sets the liquidity pool id for the request.
    ///
    /// # Arguments
    /// * `liquidity_pool_id` - A `String` value representing the liquidity pool id.
    ///
    pub fn set_transaction_hash(
        self,
        transaction_hash: String,
    ) -> EffectForTransactionRequest {
        EffectForTransactionRequest {
            transaction_hash: Some(transaction_hash),
            ..self
        }
    }
}

impl Request for EffectForTransactionRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.transaction_hash
                .as_ref()
                .map(|l| format!("transaction_hash={}", l)),
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::EFFECTS_PATH,
            self.get_query_parameters()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BuildQueryParametersExt;

    #[test]
    fn test_effects_for_liquidity_pools_request() {
        let request = EffectForTransactionRequest::new()
            .set_transaction_hash("transaction_hash".to_string())
            .set_cursor(1)
            .unwrap()
            .set_limit(10)
            .unwrap()
            .set_order(Order::Asc)
            .unwrap();

        let url = request.build_url("https://horizon-testnet.stellar.org");
        let query_parameters = vec![
            Some("transaction_hash=transaction_hash".to_string()),
            Some("cursor=1".to_string()),
            Some("limit=10".to_string()),
            Some("order=asc".to_string()),
        ]
        .build_query_parameters();

        assert_eq!(
            url,
            "https://horizon-testnet.stellar.org/effects?transaction_hash=transaction_hash&cursor=1&limit=10&order=asc"
        );
        assert_eq!(query_parameters, "?transaction_hash=transaction_hash&cursor=1&limit=10&order=asc");
    }
}