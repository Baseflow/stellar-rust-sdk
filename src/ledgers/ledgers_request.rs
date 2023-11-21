use crate::{BuildQueryParametersExt, models::*};

use super::super::Order;

#[derive(Default)]
pub struct LedgersRequest {
    /// The cursor for the page
    cursor: Option<u32>,
    /// The maximum number of records to return
    limit: Option<u32>,
    /// The order of the records
    order: Option<Order>,
}

impl LedgersRequest {
    pub fn new() -> Self {
        LedgersRequest::default()
    }
}

impl Request for LedgersRequest {
    fn get_path(&self) -> &str {
        "/ledgers"
    }

    fn get_query_parameters(&self) -> String {
        let mut query_parameters = vec![];

        if let Some(cursor) = &self.cursor {
            query_parameters.push(format!("cursor={}", cursor));
        }

        if let Some(limit) = &self.limit {
            query_parameters.push(format!("limit={}", limit));
        }

        if let Some(order) = &self.order {
            query_parameters.push(format!("order={}", order));
        }

        query_parameters.join("&")
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::LEDGERS_PATH,
            self.get_query_parameters()
        )
    }
}

impl LedgersRequest {
    /// Sets the cursor
    /// # Arguments
    /// * `cursor` - The cursor
    /// # Returns
    /// The request object
    /// [AllLedgersRequest](struct.AllLedgersRequest.html)
    pub fn set_cursor(self, cursor: u32) -> Result<LedgersRequest, String>{
        if cursor < 1 {
            return Err("cursor must be greater than or equal to 1".to_string());
        }

        Ok(
            LedgersRequest {
                cursor: Some(cursor),
                limit: self.limit,
                order: self.order
            }
        )
    }

    /// Sets the limit
    /// # Arguments
    /// * `limit` - The limit
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_limit(self, limit: u32) -> Result<LedgersRequest, String> {
        if limit < 1 || limit > 200 {
            return Err("limit must be between 1 and 200".to_string());
        }

        Ok(
            LedgersRequest {
                cursor: self.cursor,
                limit: Some(limit),
                order: self.order
            }
        )
    }

    /// Sets the order
    /// # Arguments
    /// * `order` - The order
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_order(&mut self, order: Order) -> LedgersRequest {
        LedgersRequest {
            cursor: self.cursor,
            limit: self.limit,
            order: Some(order)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledgers_request() {
        let request = LedgersRequest::new();
        assert_eq!(
            request.build_url("https://horizon-testnet.stellar.org"),
            "https://horizon-testnet.stellar.org/ledgers"
        );
    }
}
