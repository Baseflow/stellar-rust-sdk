use stellar_xdr::curr::SurveyRequestMessage;

use crate::{models::{Order, Request}, BuildQueryParametersExt};

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
/// use stellar_rs::effects::effects_for_transaction_request::EffectForTransactionRequest;
/// use stellar_rs::models::*;
///
/// let request = EffectForTransactionRequest::new()
///     .set_transaction_hash("transaction_hash".to_string())
///     .set_cursor(1234).unwrap()
///     .set_limit(20).unwrap()
///     .set_order(Order::Desc);
///
/// // The request can now be used with a Horizon client to fetch effects.
/// ```
#[derive(Default)]
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

    /// Sets the cursor for pagination.
    ///
    /// # Arguments
    /// * `cursor` - A `u32` value pointing to a specific location in a collection of responses.
    ///
    pub fn set_cursor(self, cursor: u32) -> Result<EffectForTransactionRequest, String> {
        if cursor < 1 {
            return Err("cursor must be greater than or equal to 1".to_string());
        }

        Ok(EffectForTransactionRequest {
            cursor: Some(cursor),
            ..self
        })
    }

    /// Sets the maximum number of records to return.
    ///
    /// # Arguments
    /// * `limit` - A `u8` value specifying the maximum number of records. Range: 1 to 200. Defaults to 10.
    ///
    pub fn set_limit(self, limit: u8) -> Result<EffectForTransactionRequest, String> {
        if limit < 1 || limit > 200 {
            return Err("limit must be between 1 and 200".to_string());
        }

        Ok(EffectForTransactionRequest {
            limit: Some(limit),
            ..self
        })
    }

    /// Sets the order of the returned records.
    ///
    /// # Arguments
    /// * `order` - An [`Order`] enum value specifying the order (ascending or descending).
    ///
    pub fn set_order(self, order: Order) -> EffectForTransactionRequest {
        EffectForTransactionRequest {
            order: Some(order),
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
            .set_order(Order::Asc);

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
