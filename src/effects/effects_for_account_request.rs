use crate::{models::{Order, Request}, BuildQueryParametersExt};

#[derive(Default)]
pub struct EffectsForAccountRequest {
    /// The accounts public id
    account_id: Option<String>,

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

impl EffectsForAccountRequest {
    /// Creates a new `LedgersRequest` with default parameters.
    pub fn new() -> Self {
        EffectsForAccountRequest::default()
    }

    /// Sets the account id for the request.
    /// 
    /// # Arguments
    /// * `account_id` - A `String` value representing the account id.
    /// 
    pub fn set_account_id(self, account_id: String) -> EffectsForAccountRequest {
        EffectsForAccountRequest {
            account_id: Some(account_id),
            ..self
        }
    }

    /// Sets the cursor for pagination.
    ///
    /// # Arguments
    /// * `cursor` - A `u32` value pointing to a specific location in a collection of responses.
    ///
    pub fn set_cursor(self, cursor: u32) -> Result<EffectsForAccountRequest, String> {
        if cursor < 1 {
            return Err("cursor must be greater than or equal to 1".to_string());
        }

        Ok(EffectsForAccountRequest {
            cursor: Some(cursor),
            ..self
        })
    }

    /// Sets the maximum number of records to return.
    ///
    /// # Arguments
    /// * `limit` - A `u8` value specifying the maximum number of records. Range: 1 to 200. Defaults to 10.
    ///
    pub fn set_limit(self, limit: u8) -> Result<EffectsForAccountRequest, String> {
        if limit < 1 || limit > 200 {
            return Err("limit must be between 1 and 200".to_string());
        }

        Ok(EffectsForAccountRequest {
            limit: Some(limit),
            ..self
        })
    }

    /// Sets the order of the returned records.
    ///
    /// # Arguments
    /// * `order` - An [`Order`] enum value specifying the order (ascending or descending).
    ///
    pub fn set_order(self, order: Order) -> EffectsForAccountRequest {
        EffectsForAccountRequest {
            order: Some(order),
            ..self
        }
    }
}

impl Request for EffectsForAccountRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.account_id.as_ref().map(|a| format!("account={}", a)),
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

    #[test]
    fn test_ledgers_request() {
        let request = EffectsForAccountRequest::new();
        assert_eq!(
            request.build_url("https://horizon-testnet.stellar.org"),
            "https://horizon-testnet.stellar.org/effects"
        );
    }

    #[test]
    fn test_ledgers_request_with_params() {
        let request = EffectsForAccountRequest::new()
            .set_account_id("GBL3QJ2MB3KJ7YV7YVXJ5ZL5V6Z5ZL5V6Z5ZL5V6Z5ZL5V6Z5ZL5V6Z".to_string())
            .set_cursor(1)
            .unwrap()
            .set_limit(10)
            .unwrap()
            .set_order(Order::Desc);
        assert_eq!(
            request.build_url("https://horizon-testnet.stellar.org"),
            "https://horizon-testnet.stellar.org/effects?account=GBL3QJ2MB3KJ7YV7YVXJ5ZL5V6Z5ZL5V6Z5ZL5V6Z5ZL5V6Z5ZL5V6Z&cursor=1&limit=10&order=desc"
        );
    }
}
