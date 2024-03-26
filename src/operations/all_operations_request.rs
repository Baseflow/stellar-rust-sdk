use crate::models::{IncludeFailed, Order, Request};

#[derive(Default)]
pub struct AllOperationsRequest {
    /// A pointer to a specific location in a collection of responses, derived from the
    ///   `paging_token` value of a record. Used for pagination control in the API response.
    cursor: Option<u32>,

    /// Specifies the maximum number of records to be returned in a single response.
    ///   The range for this parameter is from 1 to 200. The default value is set to 10.
    limit: Option<u8>,

    /// Determines the [`Order`] of the records in the response. Valid options are [`Order::Asc`] (ascending)
    ///   and [`Order::Desc`] (descending). If not specified, it defaults to ascending.
    order: Option<Order>,

    /// A boolean value that determines whether to include failed operations in the response.
    include_failed: Option<IncludeFailed>,
}

impl AllOperationsRequest {
    pub fn new() -> Self {
        AllOperationsRequest::default()
    }

    /// Sets the cursor for pagination.
    ///
    /// # Arguments
    /// * `cursor` - A `u32` value pointing to a specific location in a collection of responses.
    ///
    pub fn set_cursor(self, cursor: u32) -> Result<AllOperationsRequest, String> {
        if cursor < 1 {
            return Err("cursor must be greater than or equal to 1".to_string());
        }

        Ok(AllOperationsRequest {
            cursor: Some(cursor),
            ..self
        })
    }

    /// Sets the maximum number of records to return.
    ///
    /// # Arguments
    /// * `limit` - A `u8` value specifying the maximum number of records. Range: 1 to 200. Defaults to 10.
    ///
    pub fn set_limit(self, limit: u8) -> Result<AllOperationsRequest, String> {
        if limit < 1 || limit > 200 {
            return Err("limit must be between 1 and 200".to_string());
        }

        Ok(AllOperationsRequest {
            limit: Some(limit),
            ..self
        })
    }

    /// Sets the order of the returned records.
    ///
    /// # Arguments
    /// * `order` - An [`Order`] enum value specifying the order (ascending or descending).
    ///
    pub fn set_order(self, order: Order) -> AllOperationsRequest {
        AllOperationsRequest {
            order: Some(order),
            ..self
        }
    }

    /// Sets whether to include failed operations in the response.
    ///
    /// # Arguments
    /// * `include_failed` - A boolean value that determines whether to include failed operations in the response.
    ///
    pub fn set_include_failed(self, include_failed: IncludeFailed) -> AllOperationsRequest {
        AllOperationsRequest {
            include_failed: Some(include_failed),
            ..self
        }
    }
}

impl Request for AllOperationsRequest {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
            self.include_failed
                .as_ref()
                .map(|f| format!("include_failed={}", f)),
        ]
        .iter()
        .flatten()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("&")
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}?{}",
            base_url,
            super::OPERATIONS_PATH,
            self.get_query_parameters()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Order;

    #[test]
    fn test_all_operations_request() {
        let request = AllOperationsRequest::new()
            .set_cursor(1)
            .unwrap()
            .set_limit(10)
            .unwrap()
            .set_order(Order::Desc)
            .set_include_failed(IncludeFailed::True);

        assert_eq!(
            request.get_query_parameters(),
            "cursor=1&limit=10&order=desc&include_failed=true"
        );
    }
}
