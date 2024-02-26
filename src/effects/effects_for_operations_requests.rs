use crate::{models::{Order, Request}, BuildQueryParametersExt};

#[derive(Default)]
pub struct EffectsForOperationRequest {
    /// The unique identifier of the operation.
    operation_id: Option<String>,

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

impl EffectsForOperationRequest {
    /// Creates a new `EffectsForOperationRequest` with default parameters.
    pub fn new() -> Self {
        EffectsForOperationRequest::default()
    }

    /// Sets the operation_id id for the request.
    ///
    /// # Arguments
    /// * `operation_id` - A `String` value representing the operation id.
    ///
    pub fn set_operation_id(self, operation_id: String) -> EffectsForOperationRequest {
        EffectsForOperationRequest {
            operation_id: Some(operation_id),
            ..self
        }
    }

    /// Sets the cursor for pagination.
    ///
    /// # Arguments
    /// * `cursor` - A `u32` value pointing to a specific location in a collection of responses.
    ///
    pub fn set_cursor(self, cursor: u32) -> Result<EffectsForOperationRequest, String> {
        if cursor < 1 {
            return Err("cursor must be greater than or equal to 1".to_string());
        }

        Ok(EffectsForOperationRequest {
            cursor: Some(cursor),
            ..self
        })
    }

    /// Sets the maximum number of records to return.
    ///
    /// # Arguments
    /// * `limit` - A `u8` value specifying the maximum number of records. Range: 1 to 200. Defaults to 10.
    ///
    pub fn set_limit(self, limit: u8) -> Result<EffectsForOperationRequest, String> {
        if limit < 1 || limit > 200 {
            return Err("limit must be between 1 and 200".to_string());
        }

        Ok(EffectsForOperationRequest {
            limit: Some(limit),
            ..self
        })
    }

    /// Sets the order of the returned records.
    ///
    /// # Arguments
    /// * `order` - An [`Order`] enum value specifying the order (ascending or descending).
    ///
    pub fn set_order(self, order: Order) -> EffectsForOperationRequest {
        EffectsForOperationRequest {
            order: Some(order),
            ..self
        }
    }
}

impl Request for EffectsForOperationRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.operation_id.as_ref().map(|a| format!("operation={}", a)),
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
    use crate::models::Order;

    use super::*;

    #[test]
    fn test_effects_for_account_request() {
        let request = EffectsForOperationRequest::new();
        assert_eq!(
            request.build_url("https://horizon-testnet.stellar.org"),
            "https://horizon-testnet.stellar.org/effects"
        );
    }

    // #[test]
    // fn test_effects_for_operations_request_with_params() {
    //     let request = EffectsForOperationRequest::new()
    //         .set_operation_id("0000000459561504769".to_string())
    //         .set_cursor(1)
    //         .unwrap()
    //         .set_limit(10)
    //         .unwrap()
    //         .set_order(Order::Desc);
    //     assert_eq!(
    //         request.build_url("https://horizon-testnet.stellar.org"),
    //         "https://horizon-testnet.stellar.org/effects?operation=0000000459561504769&cursor=1&limit=10&order=desc"
    //     );
    // }
    
    #[test]
    fn test_effects_for_operations_request_set_limit() {
        let invalid_limit: u8 = 255;

        let request = EffectsForOperationRequest::new()
            .set_limit(invalid_limit);

        assert!(request.is_err());
    }

    #[test]
    fn test_effects_for_operations_request_set_cursor() {
        let invalid_cursor = 0;

        let request = EffectsForOperationRequest::new()
            .set_cursor(invalid_cursor);

        assert!(request.is_err());
    }
}