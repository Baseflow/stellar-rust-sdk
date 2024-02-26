use crate::{
    models::{Order, Request},
    BuildQueryParametersExt,
};

#[derive(Default)]
pub struct EffectsForOperationRequest {
    operation_id: Option<String>,
    cursor: Option<u32>,
    limit: Option<u8>,
    order: Option<Order>,
}

impl EffectsForOperationRequest {
    pub fn new() -> EffectsForOperationRequest {
        EffectsForOperationRequest::default()
    }

    pub fn set_operation_id(self, operation_id: &str) -> EffectsForOperationRequest {
        EffectsForOperationRequest {
            operation_id: Some(operation_id.to_string()),
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
            self.operation_id
                .as_ref()
                .map(|l| format!("operation_id={}", l)),
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ]
        .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!("{}/{}{}", base_url, super::EFFECTS_PATH, self.get_query_parameters(),)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_query_parameters() {
        let request = EffectsForOperationRequest::new()
            .set_operation_id("123")
            .set_cursor(1)
            .unwrap()
            .set_limit(10)
            .unwrap()
            .set_order(Order::Asc);

        let query_parameters = request.get_query_parameters();
        assert_eq!(
            query_parameters,
            "?operation_id=123&cursor=1&limit=10&order=asc"
        );
    }

    #[test]
    fn test_build_url() {
        let request = EffectsForOperationRequest::new();
        let base_url = "https://horizon-testnet.stellar.org";
        let url = request.build_url(base_url);
        assert_eq!(
            url,
            "https://horizon-testnet.stellar.org/effects"
        );
    }
}
